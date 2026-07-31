#!/usr/bin/env python3
"""
Script to remove emojis from Python docstrings to fix syntax errors.
"""

import os
import re
import glob
from pathlib import Path

def remove_emojis_from_docstrings(file_path):
    """Remove emojis from docstrings in Python files"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern to match emojis
        emoji_pattern = re.compile(
            r'[\U0001F600-\U0001F64F\U0001F300-\U0001F5FF\U0001F680-\U0001F6FF\U0001F1E0-\U0001F1FF'
            r'\U00002702-\U000027B0\U000024C2-\U0001F251]'
        )
        
        # Remove emojis from docstring content (between triple quotes)
        lines = content.split('\n')
        filtered_lines = []
        
        for line in lines:
            # Check if we're in a docstring
            if '"""' in line:
                # Remove emojis from this line
                filtered_line = emoji_pattern.sub('', line)
                filtered_lines.append(filtered_line)
            else:
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
            "session.json", ".repo/⚡️/🤖️/", ".repo/files.json"
        ]):
            continue
        
        total_files += 1
        if remove_emojis_from_docstrings(full_path):
            modified_files += 1
            print(f"Modified: {full_path}")
    
    print(f"\nSummary:")
    print(f"Total files processed: {total_files}")
    print(f"Files modified: {modified_files}")

if __name__ == "__main__":
    main()
