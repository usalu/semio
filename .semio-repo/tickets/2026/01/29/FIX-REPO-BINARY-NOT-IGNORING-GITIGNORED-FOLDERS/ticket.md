# Ticket

## Todos

## Changes

## Log

## Summary

Fixed GetFolderChildren() in ./semio-repo/cli/main.go to apply gitignore filtering. The function was missing the GetGitIgnoredSet() call that ToolFolderList, ToolFolderTree, and GetFolderFiles all had. Now folder children are collected as candidates first, checked against git check-ignore, and filtered before being returned.
