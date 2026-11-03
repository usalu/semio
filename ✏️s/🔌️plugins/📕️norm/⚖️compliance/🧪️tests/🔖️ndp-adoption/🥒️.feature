@capability-norm-ndp-adoption @oracle-norm-ndp-adoption-python-independent
Feature: The shared NDP registry is the single source of every nationally determined parameter
  The registry `📜️annex-and-tables/🔖️ndp/🔣️.json` holds the EN recommended and the NA-DE value of every parameter the
  artifacts EN 1990 to EN 1999 use. The independent Python oracle `🐍️.py` reads the registry and the artifact sources
  as text and needs no build.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - Every NDP id an artifact cites is registered: Given the committed NDP registry and the artifact sources of EN 1990 to EN 1999 When the string literals "ndp.en199x.…" in the sources are collected Then each literal is a registry id or the prefix of registry ids
  - No artifact keeps a private list of nationally determined parameters: Given the artifact sources of EN 1990 to EN 1999 Then no source declares UNVERIFIED_NDP, NDP_REGISTRY, a private ndp module, UNVERIFIED, NDPS or GAMMA_M_TABLE
  - Switching the annex changes exactly the expected values: Given the committed NDP registry When the numeric entries whose EN recommended and NA-DE values differ are listed Then the list equals the reviewed fixture `🧫️fixtures/🔖️ndp-annex-differences/🔣️.json` And every code family EN 1990 to EN 1999 owns registry entries
