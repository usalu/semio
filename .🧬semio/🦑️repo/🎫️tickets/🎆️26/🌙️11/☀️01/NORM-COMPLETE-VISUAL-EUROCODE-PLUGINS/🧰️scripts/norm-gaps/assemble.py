import re
T = open("tables.md", encoding="utf-8").read()
rows = {}
for l in T.splitlines():
    m = re.match(r"\| (\S+) \| (\S+) \| (.*?) \| (missing|partial) \| (\S+) \| (.*?) \| (.*) \|$", l)
    if m:
        rows[(m.group(1), m.group(2))] = m
rank = [
 ("en1999","A-70","Shell buckling: classical elastic critical stress, cylindrical shells (EN 1999-1-5 §6-7). Stability ULS for tanks, silos, tubes."),
 ("en1999","A-61","Sheeting bending, web crippling and shear (EN 1999-1-4 §6.1). Core ULS resistance for purlins and cladding."),
 ("en1999","A-72","Welded shells with HAZ reduction on axial compression (EN 1999-1-5). ULS of welded shells."),
 ("en1999","A-12","Net-section tension N_u,Rd = 0.9 A_net f_u/γM2 with holes not modelled (6.2.3). ULS fracture of bolted members."),
 ("en1993","V-501","Stress-range derivation (5.1/5.2) from bands or rainflow history; fatigue ULS input for every detail."),
 ("en1993","V-259","Cold-formed screw and blind-rivet connections (section 8). ULS of fastened joints not modelled."),
 ("en1993","V-126","Buckling length Lcr = kL with nomogram k from G_A, G_B not modelled; buckling lengths are direct inputs (6.3.1.3). ULS stability."),
 ("en1993","V-407","Eccentric bolt groups (3.12): bolt coordinates and eccentricity missing, so group resistance is not derived. ULS joints."),
 ("en1993","V-250","Enhanced yield strength of cold-formed sections (3.2.2): f_u and corner data not carried. ULS resistance gain."),
 ("en1993","V-029","Plastic global analysis gate (5.6): class and restraint at hinges. ULS when plastic design is used."),
 ("en1992","ST-03","Strut-and-tie node capacity CCC/CCT/CTT (6.5.4); trace fragment stale, id present, verify CCC node. ULS."),
 ("en1992","T-02","Torsion links and longitudinal steel (6.3.2); trace fragment stale, id present, verify torsion-links. ULS."),
 ("en1991","E2.19","Derailment design situations I and II (EN 1991-2 6.8). Accidental ULS for rail bridges; not modelled."),
 ("en1991","E4.07","Dust explosion pressure p_max on silos (EN 1991-4). Accidental ULS; not modelled."),
 ("en1991","E2.20","Rail load groups gr11 to gr31 (Table 6.11) for traffic ULS and SLS; not modelled."),
 ("en1991","E4.06","Thermal actions on silo walls (ΔT); not modelled."),
 ("en1998","GE-04","Pore pressure increase and lateral-spreading flag (EN 1998-5 4.1.4, Annex B). Seismic ULS for liquefiable ground."),
 ("en1998","BR-10","Seismic isolation of bridges (EN 1998-2 §6). Seismic ULS; not modelled beyond spring stiffness."),
 ("en1998","BR-11","Foundation and abutment soil-structure interaction for bridges; not modelled."),
 ("en1998","GE-12","Pile in seismic ground: inertial and kinematic moments and bending resistance (EN 1998-5 5.4); partial."),
 ("en1998","SEI-53","Building isolation (T_eff, ξ_eff, d_cd) not modelled; only isolated bridges covered; seismic ULS."),
 ("en1994","C-32","Composite column fire, simple calculation model; tabulated data C-46 not modelled (EC4 F:4.3). Accidental ULS."),
 ("en1993","V-011","Ductility f_u/f_y ≥ 1.10 and ε_u ≥ 15 ε_y (3.2.2); elongation check not carried. ULS for plastic and global design."),
 ("en1991","E2.18","Aerodynamic effects from passing trains (6.7) not modelled; spec gives unverified p_k."),
 ("en1997","GI-03","CPT undrained shear strength c_u with cone factor N_kt (Annex D); partial. Geotechnical ULS input."),
]
out = ["| Rank | Code | Id | Status | Spec line | Why it ranks here |", "|---|---|---|---|---|---|"]
for i, (code, id_, why) in enumerate(rank, 1):
    m = rows.get((code, id_))
    st = m.group(4) if m else "blocked-unverified"
    ln = m.group(5) if m else "en1990-en1991-target.md:326"
    out.append("| %d | %s | %s | %s | %s | %s |" % (i, code, id_, st, ln, why))
R = "\n".join(out)
counts = open("tables.md", encoding="utf-8").read().split("\n### ")[0]
gaps = "### " + open("tables.md", encoding="utf-8").read().split("\n### ")[1]
blocked = open("blocked.md", encoding="utf-8").read()
doc = f"""# 🔍️ Norm Gap Audit, EN 1990 to EN 1999 (Wave 3, W3-NORM-GAPS)

Read-only audit. No code was edited and no build or test was run. Ticket `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`.

## Method

- Targets: every id of each code's spec-trace `specIds` (the set the trace declares as the target spec). All `specIds` appear in the target markdown (`📐️specs/*-target.md`), and no trace item is missing from `specIds`.
- Implementation evidence: the trace `items[].realisation` of `🗿️artifacts/<code>/…/🧭️spec-trace/🔣️.json`, checked against the artifact's non-test Rust code (`🧬️schema/**`, `⚖️compliance`, excluding `spec-trace`, `tests`, `showcase`, `scene`, `viewer`, `editor`, `examples`, fixtures).
  - `check` rows: a fragment is implemented when it is a literal in non-test code. A row is implemented when its quoted catalogue id (for example `"GI-02"`) is a literal in non-test code, even if the trace fragment is absent (trace stale).
  - `design-case` rows: implemented when the quoted entry id is in the catalogue.
  - `data` rows: implemented when the leading token of the data path is in non-test code.
  - `not-modelled` rows: missing, or blocked when the reason cites an unverified or printed value.
- Blocked: implemented rows whose text or target row says `[V]`, `unverified`, `provisional` or `print`, and not-modelled rows whose reason cites print or unverified data.
- Partial: a check row with some fragments unmatched, or an implemented row whose text states a scope limit ("not modelled", "not carried", "special study", "out of scope", "not covered", "not used").
- Spec line: first line of the target markdown where the id appears (table row preferred). Clause as written in the trace.
- Caveat: the trace fragments are often stale (see Findings). Status is a static check, not a behavioural test. The per-code rows below are the result of the script run; the counts were not cross-checked against the reports.

## Per-code counts

{counts.split(chr(10),1)[1] if False else chr(10).join(counts.splitlines()[0:])}

## Top 25 missing or partial items, ranked by engineering importance

Ranking order: safety-relevant ultimate-limit-state (ULS) and accidental items first (stability, fracture, plastic resistance, connections, seismic collapse), then fatigue and traffic, then SLS and design inputs. Only 22 items are "missing" in total; the list fills to 25 with partial items that carry ULS weight. EN 1991 E2.19 (derailment) is blocked in the status rules because its reason cites an unverified value, but it is a missing model and ranks here.

{R}

Blocked items with high ULS weight that are not in the top 25 because they are blocked on print: EN 1994 C-46 (tabulated fire data, F:4.2), EN 1993 V-505 (fatigue detail Tables 8.1 to 8.10), EN 1993 V-609 (advanced fire methods), EN 1996 M-05 (anchorage bond length), EN 1996 M-31 (yield-line two-way bending), EN 1998 GE-06 (seismic bearing capacity, Annex F), EN 1998 BR-06 (seismic component combination), EN 1998 AS-06 (cyclic shear capacity).

## Missing and partial items (full list, clause and spec line)

{gaps}

Partial rows marked "id literal present" have their quoted catalogue id in non-test code. They are probably implemented and the trace fragment is stale. Verify before counting them as gaps. The en1990 E90.xx partials are an example, since the STR combination is implemented under `en1990.6.4.combinations`.

## Blocked on print or unverified (ids per code)

Counted as blocked, not missing. Each needs printed-standard or NA-DE verification before the value is trusted.

{blocked}
## Findings

- 81 implemented check rows (en1997 37, en1995 24, en1996 9, en1991 6, en1990 5) match only by catalogue id; their trace fragment is absent. Plus 28 partial rows, most of which also have an id literal present. Ids are implemented under catalogue names; the trace still points at old fragment names. The trace is not a reliable gap source on its own. Fix the trace fragments rather than the code.
- 22 rows are truly missing (no fragment, no catalogue id). Most are "not modelled" in the spec itself: EN 1991 rail and silo items, EN 1998 bridge and pipe items, EN 1999 sheeting and shell buckling (A-61, A-62, A-70, A-72).
- EN 1999 sheeting and shells (A-61, A-70, A-72) are the largest ULS gap in the code set. No catalogue entry exists for them.
- EN 1993 partials (V-126, V-407, V-250, V-029) are scope limits stated in the spec text, not stale fragments.
- EN 1992 has no missing rows. Its partials (T-02, ST-03) are stale-fragment rows with id literals present.
"""
open("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🔍️audits/wave3/norm-gaps.md", "w", encoding="utf-8").write(doc)
print(len(doc))
