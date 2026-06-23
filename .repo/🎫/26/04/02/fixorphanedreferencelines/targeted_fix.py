#!/usr/bin/env python3
"""
Script to fix only the specific orphaned reference lines that cause syntax errors.
"""

import os
import re
from pathlib import Path

def fix_specific_orphaned_refs(file_path):
    """Fix only specific orphaned reference lines"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Find and remove specific patterns that are causing issues
        lines = content.split('\n')
        filtered_lines = []
        
        for i, line in enumerate(lines):
            # Skip lines that are exact matches for orphaned references
            # These are lines that start with [👤 and are not part of docstrings
            if (re.match(r'^\s*\[👤📚💻🔖🛠️]', line) and
                i > 0 and not lines[i-1].strip().endswith('"""') and
                i < len(lines) - 1 and not lines[i+1].strip().startswith('"""')):
                # This is an orphaned reference line, skip it
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
    
    # Process specific files that we know have issues
    files_to_fix = [
        "compose/py/main.py",
        "compose/engine/main.py", 
        "coda/engine/coda.py"
    ]
    
    total_files = 0
    modified_files = 0
    
    for file_path in files_to_fix:
        full_path = repo_root / file_path
        if full_path.exists():
            total_files += 1
            if fix_specific_orphaned_refs(full_path):
                modified_files += 1
                print(f"Modified: {full_path}")
    
    print(f"\nSummary:")
    print(f"Total files processed: {total_files}")
    print(f"Files modified: {modified_files}")

if __name__ == "__main__":
    main()
