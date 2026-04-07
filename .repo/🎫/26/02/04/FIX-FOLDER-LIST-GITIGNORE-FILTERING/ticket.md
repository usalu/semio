# Ticket

## Summary

Fixed the isGitIgnored function to always treat .git folders as ignored. The .git folder is special and is always excluded by git internally (not through .gitignore). Added a check at the beginning of isGitIgnored to return true for any path that is '.git' or starts with '.git/'. Now running 'folder list' hides .git folders by default, and they only appear when using the --show-ignored flag.
## Changes

## Log

## Todos

## Plan
