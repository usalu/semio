#!/bin/zsh
# N1 one-off: restores the label-edited leaf files listed in n1-label-files.txt from HEAD (read-only git show, no checkout).
cd /Users/ueli/Documents/semio || exit 1
while IFS= read -r file; do git show "HEAD:$file" > "$file"; done < .tmp-ticket/wp-n1/n1-label-files.txt
echo "restored $(wc -l < .tmp-ticket/wp-n1/n1-label-files.txt) files"
