@capability-norm-ndp-registry @oracle-norm-ndp-registry-python-independent
Feature: Unverified nationally determined parameters block a compliant verdict
  The registry `📜️annex-and-tables/🔖️ndp/🔣️.json` is the language-agnostic source of every NDP and table cell used by a
  check. The Rust kernel and the independent Python oracle `🐍️.py` read the same bytes and apply the same verdict rule.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - The registry is sorted, unique and fully provenanced: Given the committed NDP registry Then every id is unique and the ids are sorted And every entry names a code, a clause locator, a source document and a status of verified or unverified And every verified entry records a print edition
  - A check citing <ndps> with computed <computed> against limit <limit> ends as <status>: Given the committed NDP registry When a utilisation check of <computed> against <limit> cites the NDP ids <ndps> Then the check status is <status> And the report complies is <complies>
  - A minimum check of <computed> against minimum <minimum> reports utilisation <utilization>: When a minimum check of <computed> against <minimum> is evaluated Then the check status is <status> And the utilisation is <utilization>
  - A read of an NDP without a value for the annex is a localized refusal: Given the committed NDP registry And an NDP with a DE value but no EN value When the NDP is read for the annex EN Then the read yields no number And the notice is "NDP <id> has no value for annex EN" in English and "NDP <id> hat keinen Wert für den Anhang EN" in German And a check resting on the read ends as UnverifiedBasis, a pass and a failure alike, never as a default value
  - Annex-blind reads are proven equal by registry rows: Given every id read without an annex choice (`ndp_common`, `ndp_cell_common`, `normative_limit`) When the EN and DE values are compared Then both are numbers and equal
