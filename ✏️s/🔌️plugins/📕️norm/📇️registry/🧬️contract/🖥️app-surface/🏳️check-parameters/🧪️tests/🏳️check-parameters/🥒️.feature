@capability-check-parameters
@oracle-registry-table
Feature: The inspector lists the annex parameters the selected check rests on
  Executed by `🧪️tests/🦀️.rs`; the independent oracle is the NDP registry table read directly.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Listing: Given a check citing three parameters When its parameters are listed Then each shows id, symbol, clause, the value of the check's annex and a verified or unverified word in English and German
  - Unknown id: Given a check citing an id the registry does not know When its parameters are listed Then the id is shown as unverified with a dash
  - Override link: Given factor override rows of the document When a psi or gamma parameter is listed Then the last matching row (category target before empty target) is linked, and a row activates selection of its governing check
  - No parameters: Given a check without parameters Then the inspector shows no section
