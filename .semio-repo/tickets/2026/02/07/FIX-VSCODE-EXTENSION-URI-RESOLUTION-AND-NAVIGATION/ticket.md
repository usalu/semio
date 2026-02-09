---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-VSCODE-EXTENSION
---

# Ticket

## Summary

Fixed bundle tree items showing raw kind strings (e.g. 'librarysemio/js') instead of emoji-prefixed short names (e.g. '📚js'). Added bundleKindEmoji() mapping function matching the Go CLI's bundleKindEmoji() (schema→🛂, binary→⌨️️, ui→🖱️️, site→🌐, assets→🏪, library→📚). Fixed label to extract short name after '/' from full bundle name. Also removed leftover duplicate constructor/refresh in FilterTreeDataProvider. Added bundleKindEmoji tests.

## Changes

## Log

## Todos

## Plan
