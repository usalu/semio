#!/usr/bin/env python3
"""
Script to fix broken lines caused by emoji removal.
"""

import os
import re
from pathlib import Path

def fix_broken_lines(file_path):
    """Fix broken lines with mid-line breaks"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix lines that have been broken in the middle
        lines = content.split('\n')
        fixed_lines = []
        
        for line in lines:
            # Check if line ends with a partial word (indicating it was broken)
            if (line.strip() and 
                not line.strip().endswith(('.', ':', ',', ';', ')', ']', '}')) and
                len(line.strip()) > 0 and
                not line.strip().startswith('#') and
                not line.strip().startswith('//') and
                not line.strip().startswith('"""') and
                not line.strip().endswith('"""') and
                not 'def ' in line and
                not 'class ' in line and
                not 'import ' in line and
                not 'from ' in line):
                # This might be a broken line, join with next
                continue
            
            fixed_lines.append(line)
        
        new_content = '\n'.join(fixed_lines)
        
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
            if fix_broken_lines(full_path):
                modified_files += 1
                print(f"Modified: {full_path}")
    
    print(f"\nSummary:")
    print(f"Total files processed: {total_files}")
    print(f"Files modified: {modified_files}")

if __name__ == "__main__":
    main()
