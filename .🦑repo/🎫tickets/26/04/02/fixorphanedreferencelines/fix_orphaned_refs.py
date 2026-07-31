#!/usr/bin/env python3
"""
Script to fix orphaned reference lines that contain emojis after removing docstring comments.
These lines cause syntax errors in Python files.
"""

import os
import re
import glob
from pathlib import Path

def fix_orphaned_references(file_path):
    """Fix orphaned reference lines that contain emojis"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Split into lines and process
        lines = content.split('\n')
        filtered_lines = []
        
        for i, line in enumerate(lines):
            # Skip lines that are standalone reference lines with emojis (not in docstrings)
            if (re.search(r'^\s*\[👤📚💻🔖🛠️]', line) and
                not i > 0 and lines[i-1].strip().startswith('"""') and
                not i < len(lines) - 1 and lines[i+1].strip().endswith('"""')):
                continue
            filtered_lines.append(line)
        
        new_content = '\n'.join(filtered_lines)
        
        # Only write if content changed
        if new_content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Main function to process Python files"""
    repo_root = Path("/workspaces/semio")
    
    # Find all Python files
    pattern = f"**/*.py"
    file_paths = glob.glob(pattern, root_dir=repo_root, recursive=True)
    
    total_files = 0
    modified_files = 0
    
    for file_path in file_paths:
        full_path = repo_root / file_path
        
        # Skip certain directories
        if any(skip in str(full_path) for skip in [
            ".git", "node_modules", ".nx", "target", "vendor", 
            "__pycache__", ".pytest_cache", "dist", "build", ".venv"
        ]):
            continue
        
        # Skip session files and temporary files
        if any(skip in str(full_path) for skip in [
            "session.json", ".repo/⚡/🤖/", ".repo/files.json"
        ]):
            continue
        
        total_files += 1
        if fix_orphaned_references(full_path):
            modified_files += 1
            print(f"Modified: {full_path}")
    
    print(f"\nSummary:")
    print(f"Total files processed: {total_files}")
    print(f"Files modified: {modified_files}")

if __name__ == "__main__":
    main()
