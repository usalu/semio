# Dense Bifurcation Memory Read-Only Audit — 2026-10-04

No production edit or compilation. Inspected current native bifurcation body, retained diagnostic logs and installed Tectonic bundle PGF implementation.

## Observations

Current scientific-mathematics.sty lines649–684 defines one reusable dot protocol, then per retained point creates a pgfscope, shifts the transform, constructs a radius0.09mm circle for bounding-box accounting, syncs the low-level transform, appends the reusable dot protocol and closes scope. Buffer flush occurs per column. Draw density remains 121 columns ×60 kept orbits, with 120 transient iterations and fill opacity0.55.

Retained phase-diagnostic orbits.log reaches column109 then exhausts TeX main memory5000000 while expanding l__regex_curr_submatches_tl. Retained regex-diagnostic log also exhausts memory in l__regex_matched_analysis_tl despite a precompiled diagnostic numeric regex. These identify the allocation that encountered exhaustion; they do not prove regex retention causes the growth. The green4 source-library texput.log is only a separate missing-h1.tex job and cannot certify the native bifurcation result.

The actual loaded backend in phase log is PGF3.1.9a pgfsys-xetex.def, importing pgfsys-dvipdfmx.def. Installed bundle root: C:/Users/Ueli/AppData/Local/TectonicProject/Tectonic/bundles/data/6ffe055852f8faf66c0acbe1a7fb27f87b869a90bad1204f3bf4d9683f597c7c.

pgfsysprotocol.code.tex lines32–35 appends buffered material globally; lines64–66 invoke then globally reset currentprotocol to empty. Thus the existing per-column flush does clear the global PGF macro buffer. pgfsys-dvipdfmx.def line30 implements each invocation with a TeX special containing pdf:code. Such specials remain in the unshipped picture/page node list; flushing the macro does not release their payloads. Column batching reduces node overhead and repeated macro growth but still retains the total expanded drawing commands until shipout. This is a source-based mechanism inference, not measured node-memory attribution.

pgfsys.code.tex lines1653–1663 implements generic defobject as a globally stored protocol macro; useobject lines1677–1686 invokes that stored protocol. This backend fallback replays all circle/path tokens per point rather than invoking a PDF XObject. Reusable generic PGF objects alone therefore do not compress the retained special payload. No defobject override was found in the inspected dvipdfmx definitions.

## Useful Next Isolation

The active owner should compare column batching against a true backend PDF XObject/reference mechanism, preserving the exact circle geometry and alpha. A backend-native object reference can reduce per-point special payload; generic protocol reuse cannot. Keep the generic/native boundary explicit and cover all supported native drivers before making a permanent implementation choice. A minimal stock probe with geometry reporting disabled can distinguish drawing-payload retention from JSON regex/report overhead. Report_values delegates to probe_geometry:nx; geometry emission is guarded, while nx arguments expand before that guard. Therefore even disabled probe calls retain evaluation cost, but no accumulation was demonstrated here.

No compile success, memory repair or full catalogue pass is claimed.
