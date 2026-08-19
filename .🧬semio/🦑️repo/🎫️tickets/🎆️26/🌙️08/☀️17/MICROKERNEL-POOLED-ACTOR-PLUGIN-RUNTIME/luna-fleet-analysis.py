#!/usr/bin/env python3
"""
Analyze fleet crates for readiness to migrate to pooled-actor architecture.

Strategy:
1. Find all Cargo.toml files under ✏️s/🔌️plugins (these are the actual packages)
2. For each package, scan its .rs files for async/sync/dyn traits/tags
3. Analyze dependencies and readiness
"""

import os
import re
import json
from pathlib import Path
from collections import defaultdict
from typing import Dict, List, Tuple, Optional

BASE = Path("/Users/ueli/Documents/semio")
FLEET_BASE = BASE / "✏️s"

def find_all_cargo_packages():
    """Find ALL Cargo.toml files under plugins (both main and extensions)."""
    packages = []

    # Walk all directories under plugins
    plugins_dir = FLEET_BASE / "🔌️plugins"
    for cargo_toml_path in plugins_dir.rglob("Cargo.toml"):
        # Get the package info
        with open(cargo_toml_path) as f:
            name = "unknown"
            for line in f:
                if line.startswith('name ='):
                    name = line.split('"')[1]
                    break

        # Determine the crate path (parent of Cargo.toml)
        crate_path = cargo_toml_path.parent

        # Determine the crate root (for scanning all .rs files)
        # Walk up to find the plugin/extension root
        parts = crate_path.parts
        plugin_root = None
        if '🔌️plugins' in parts:
            idx = parts.index('🔌️plugins')
            plugin_root = Path(*parts[:idx+2])  # Up to the plugin dir itself

        packages.append({
            'name': name,
            'cargo_toml': cargo_toml_path,
            'crate_path': crate_path,
            'plugin_root': plugin_root or crate_path,
        })

    return packages

def extract_crate_kind(cargo_toml_path):
    """Determine if this is a plugin, extension, or module."""
    parts = str(cargo_toml_path).split('/')
    if '🧩️extensions' in parts:
        return 'extension'
    elif '🔨️modules' in parts:
        return 'module'
    else:
        return 'plugin'

def count_lines_in_file(path: Path) -> int:
    """Count lines in a file."""
    try:
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            return len(f.readlines())
    except:
        return 0

def analyze_rust_crate(crate_path: Path, plugin_root: Path) -> Dict:
    """Analyze a single crate for readiness metrics."""
    result = {
        'async_fns': 0,
        'sync_fns': 0,
        'test_async_fns': 0,
        'dyn_traits': defaultdict(int),
        'async_exception_tags': 0,
        'has_descriptor_json': False,
        'has_descriptor_semio': False,
        'rs_files': [],
        'test_fn_count': 0,
        'total_lines': 0,
    }

    # Check for descriptor files
    for check_path in [crate_path, plugin_root]:
        if (check_path / "🔣️descriptor.json").exists():
            result['has_descriptor_json'] = True
        if (check_path / "🛂️descriptor.semio").exists():
            result['has_descriptor_semio'] = True

    # Find all .rs files rooted at plugin_root
    rs_files = set()
    for parent in [plugin_root, crate_path]:
        if parent.exists():
            for rs_file in parent.rglob("*.rs"):
                rs_files.add(rs_file)

    # Analyze each file
    for rs_file in sorted(rs_files):
        result['rs_files'].append(str(rs_file))
        lines_in_file = count_lines_in_file(rs_file)
        result['total_lines'] += lines_in_file

        try:
            with open(rs_file, 'r', encoding='utf-8', errors='ignore') as f:
                lines = f.readlines()

                for i, line in enumerate(lines):
                    # Skip doc/line comments
                    stripped = line.strip()
                    if stripped.startswith('//') or stripped.startswith('/*'):
                        continue

                    # Count test functions
                    if re.search(r'#\[test\]|#\[tokio::test\]', line):
                        result['test_fn_count'] += 1

                    # Check if this is part of test function body (previous 5 lines)
                    is_test = False
                    if i >= 1:
                        for prev_i in range(max(0, i-10), i):
                            if re.search(r'#\[test\]|#\[tokio::test\]', lines[prev_i]):
                                is_test = True
                                break

                    # Count async fn
                    if re.search(r'\basync\s+fn\b', line):
                        if is_test:
                            result['test_async_fns'] += 1
                        else:
                            result['async_fns'] += 1

                    # Count plain fn (not async, not const, not cfg)
                    # Careful: match "fn identifier(" but not "Fn(" or in comments
                    if re.search(r'\bfn\s+[a-zA-Z_]\w*\s*\(', line):
                        if 'async' not in line:  # Not async
                            before = '\n'.join(lines[max(0, i-10):i])
                            if not any(skip in before for skip in
                                      ['#[test]', 'const fn', '#[cfg', 'extern "C"', '#[proc_macro']):
                                result['sync_fns'] += 1

                    # Count 🚫️async: exception tags
                    if '🚫️async:' in line:
                        result['async_exception_tags'] += 1

                    # Find dyn trait usage
                    dyn_matches = re.findall(r'\bdyn\s+([A-Za-z_][A-Za-z0-9_:<>, ]*\b)', line)
                    for match in dyn_matches:
                        trait_name = match.split()[0].split('+')[0].split(':')[0].strip()
                        # Exclude stdlib traits
                        stdlib_traits = {
                            'Future', 'Fn', 'FnMut', 'FnOnce', 'Error', 'Display', 'Debug',
                            'Send', 'Sync', 'Any', 'Iterator', 'IntoIterator', 'From', 'Into',
                            'Clone', 'Copy', 'Eq', 'PartialEq', 'Ord', 'PartialOrd', 'Default',
                            'Drop', 'Deref', 'DerefMut', 'AsRef', 'AsMut', 'Borrow',
                            'BorrowMut', 'Hash', 'Sized', 'Unpin', 'Unwind'
                        }
                        if trait_name not in stdlib_traits and ':' not in trait_name[:5]:
                            result['dyn_traits'][trait_name] += 1

        except Exception:
            pass  # Silently skip unparseable files

    return result

def check_has_describe_target(cargo_toml: Path) -> bool:
    """Check if Cargo.toml has a describe target."""
    try:
        with open(cargo_toml) as f:
            content = f.read()
            return ('[[bin]]' in content and 'describe' in content) or 'describe' in content
    except:
        return False

def analyze_all_crates():
    """Analyze all crates."""
    packages = find_all_cargo_packages()
    print(f"Found {len(packages)} packages")

    analysis = []
    for pkg in packages:
        crate_analysis = analyze_rust_crate(pkg['crate_path'], pkg['plugin_root'])
        crate_analysis['name'] = pkg['name']
        crate_analysis['kind'] = extract_crate_kind(pkg['cargo_toml'])
        crate_analysis['has_describe'] = check_has_describe_target(pkg['cargo_toml'])
        crate_analysis['rs_file_count'] = len(crate_analysis['rs_files'])

        # Async ratio
        total_fns = crate_analysis['async_fns'] + crate_analysis['sync_fns']
        crate_analysis['async_ratio'] = (crate_analysis['async_fns'] / total_fns * 100) if total_fns > 0 else 0

        # Repair effort heuristic
        repair_effort = 0
        repair_effort += crate_analysis['sync_fns'] * 8  # Convert sync to async
        repair_effort += sum(crate_analysis['dyn_traits'].values()) * 15  # Remove dyn traits
        repair_effort += crate_analysis['test_async_fns'] * 5  # Test issues
        repair_effort += crate_analysis['rs_file_count'] * 0.1  # File complexity
        crate_analysis['repair_effort'] = repair_effort

        analysis.append(crate_analysis)

    # Sort by repair effort
    analysis.sort(key=lambda x: x['repair_effort'])

    return analysis

if __name__ == "__main__":
    analysis = analyze_all_crates()

    # Print summary
    print("\nFirst 20 (easiest):")
    for crate in analysis[:20]:
        dyn_count = sum(crate['dyn_traits'].values()) if crate['dyn_traits'] else 0
        print(f"  {crate['name']:35s} ({crate['kind']:9s}): async={crate['async_fns']:4d}, "
              f"sync={crate['sync_fns']:3d}, dyn={dyn_count:2d}, effort={crate['repair_effort']:6.1f}")

    print(f"\nTotal: {len(analysis)} packages")

    # Last 20 (hardest)
    print("\nLast 20 (hardest):")
    for crate in analysis[-20:]:
        dyn_count = sum(crate['dyn_traits'].values()) if crate['dyn_traits'] else 0
        print(f"  {crate['name']:35s} ({crate['kind']:9s}): async={crate['async_fns']:4d}, "
              f"sync={crate['sync_fns']:3d}, dyn={dyn_count:2d}, effort={crate['repair_effort']:6.1f}")

    # Verify count
    plugins_only = [c for c in analysis if c['kind'] == 'plugin']
    extensions_only = [c for c in analysis if c['kind'] == 'extension']
    modules_only = [c for c in analysis if c['kind'] == 'module']
    print(f"\nBreakdown: {len(plugins_only)} plugins, {len(extensions_only)} extensions, {len(modules_only)} modules")
