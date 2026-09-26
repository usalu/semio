@capability-en1997-1-mutate
@oracle-en1997-1-python-independent
Feature: EN 1997 mutation kind catalog parity
  Wave C rebuilt the closed vocabulary to 20 hierarchical kinds. Full per-kind
  apply/inverse cross-checks live under each leaf `🧪tests/` directory; this
  feature tracks catalog length parity between the Rust `KINDS` table, the
  committed oracle manifest, and the independent Python catalog probe.

  Scenario: catalog length is twenty
    Given the committed en1997-1-any oracle mutation list
    Then it names exactly 20 kinds
