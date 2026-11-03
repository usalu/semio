@capability-i18n-census
@oracle-source-scan
Feature: Every user-visible string of the norm plugin is bilingual
  The census parses the Rust sources of the plugin; no default language exists, so English and German must both be present.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Complete pairs: Given source text with complete bilingual constructors When the census scans it Then there are no findings
  - Missing language: Given a bilingual constructor with an empty or whitespace English or German text When the census scans it Then the finding names the missing language and the line
  - Untranslated prose: Given identical English prose in both languages When the census scans it Then it is reported, while identical names and designations are accepted
  - Whole plugin: Given all non-test sources of the norm plugin When the census scans them Then more than 1000 pairs are inspected and none lacks English or German
