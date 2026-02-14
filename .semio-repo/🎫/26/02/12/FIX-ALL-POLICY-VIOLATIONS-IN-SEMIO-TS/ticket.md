---
goal: AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/CONSISTENT-SECTIONS
---

# Ticket

## Summary

Fixed all policy breachs in semio/js/semio.ts. Started with 744 breachs (628 DEFINITION-MISSING-SUMMARY, 64 SECTION-ORPHAN-DEFINITION, 52 SECTION-MISSING-SUMMARY). Found and fixed an analyzer bug in ParseDefinitions (brace scanning for const definitions without braces incorrectly extended ranges past subsequent definitions, causing 44 false-positive orphan breachs). After the analyzer fix, 697 real breachs remained. Fixed all by: (1) wrapping imports in an Imports section, (2) wrapping utility functions in a Utilities section, (3) moving DateProperty into Attribute section, (4) adding section summaries to all 51 sections, (5) adding definition summaries and specs to all 626+ exported definitions. Final verification: 0 breachs in semio/js/semio.ts.

## Changes

## Log

## Todos

## Plan
