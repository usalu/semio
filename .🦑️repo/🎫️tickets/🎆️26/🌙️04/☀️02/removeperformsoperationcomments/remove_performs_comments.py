#!/usr/bin/env python3
"""
Script to remove all comments with the pattern "* performs the * operation"
from source code files in the compose repository.
"""

import os
import re
import glob
from pathlib import Path

def remove_performs_comments(file_path):
    """Remove comments matching the pattern '* performs the * operation'"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Patterns to match different comment styles
        # For // comments (Go, C++, TypeScript, etc.)
        go_pattern = r'^\s*//\s*\w+\s+performs the \w+ operation\.?\s*$'
        
        # For Go method comments with "on the Type"
        go_method_pattern = r'^\s*//\s*\w+\s+performs the [\w\s]+ operation on the \w+\.?\s*$'
        
        # For Go comments with spaces in operation names
        go_spaced_pattern = r'^\s*//\s*\w+\s+performs the [\w\s]+ operation\.?\s*$'
        
        # For /// comments (Rust doc comments)
        rust_pattern = r'^\s*///\s*\w+\s+performs the \w+ operation\.?\s*$'
        
        # For # comments (Python)
        python_pattern = r'^\s*#\s*\w+\s+performs the \w+ operation\.?\s*$'
        
        # For Python docstring patterns with triple quotes and underscores
        python_docstring_pattern = r'^\s*"""*_?\w+_?\s+performs the _?\w+_? operation\.?\s*$'
        
        # For JSDoc comments (TypeScript) - with * prefix
        jsdoc_pattern = r'^\s*\*\s*Performs the \w+ operation\.?\s*$'
        
        # For JSDoc comments without * prefix
        jsdoc_no_star_pattern = r'^\s*Performs the \w+ operation\.?\s*$'
        
        # For inline code patterns (like in Go code generation) - be more specific
        inline_pattern = r'^.*specText := defName \+ " performs the " \+ defName \+ " operation\."$'
        
        # For Rust XML-style documentation tags
        rust_xml_pattern = r'^\s*///\s*<summary>\w+\s+performs the \w+ operation\.</summary>\s*$'
        
        # Split into lines and filter out matching lines
        lines = content.split('\n')
        filtered_lines = []
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Check if line matches any of the patterns
            if (re.match(go_pattern, line) or 
                re.match(go_method_pattern, line) or
                re.match(go_spaced_pattern, line) or
                re.match(rust_pattern, line) or 
                re.match(python_pattern, line) or
                re.match(python_docstring_pattern, line) or
                re.match(jsdoc_pattern, line) or
                re.match(jsdoc_no_star_pattern, line) or
                re.match(rust_xml_pattern, line) or
                re.search(inline_pattern, line)):  # Use search for inline pattern
                
                # Skip this line
                i += 1
                
                # Also skip the next line if it contains a reference with emojis
                if i < len(lines) and re.search(r'[👤️📚️💻️🔖️🛠️]', lines[i]):
                    i += 1
                
                continue
            
            filtered_lines.append(line)
            i += 1
        
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
    """Main function to process all source files"""
    repo_root = Path("/workspaces/semio")
    
    # File extensions to process
    extensions = ["*.go", "*.rs", "*.ts", "*.py"]
    
    total_files = 0
    modified_files = 0
    
    for ext in extensions:
        # Find all files with this extension
        pattern = f"**/{ext}"
        file_paths = glob.glob(pattern, root_dir=repo_root, recursive=True)
        
        for file_path in file_paths:
            full_path = repo_root / file_path
            
            # Skip certain directories
            if any(skip in str(full_path) for skip in [
                ".git", "node_modules", ".nx", "target", "vendor", 
                "__pycache__", ".pytest_cache", "dist", "build"
            ]):
                continue
            
            # Skip session files and temporary files
            if any(skip in str(full_path) for skip in [
                "session.json", ".repo/⚡️/🤖️/", ".repo/files.json"
            ]):
                continue
            
            total_files += 1
            if remove_performs_comments(full_path):
                modified_files += 1
                print(f"Modified: {full_path}")
    
    print(f"\nSummary:")
    print(f"Total files processed: {total_files}")
    print(f"Files modified: {modified_files}")

if __name__ == "__main__":
    main()
