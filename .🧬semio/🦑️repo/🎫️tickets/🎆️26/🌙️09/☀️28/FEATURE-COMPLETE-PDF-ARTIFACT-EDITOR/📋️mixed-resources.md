# Mixed Text and Image Resources in Large PDFs

The 38,459-byte specimen combines an eight-bit sampled image with Helvetica text on all four pages. It covers an inherited direct dictionary, a page-local direct dictionary, an indirect dictionary and reuse of the inherited resource on a later page. MuPDF recovered all four authored text lines at the expected baseline and matched the packed image samples and rendered pixels on every page.

The live controlled importer refused the specimen at resource qualification. The staged fix qualifies one font and one image binding together and sends them through the existing bounded cursors. The page program sees both bindings. The admitted operator role records the exact font context path, including the Resources entry for a direct dictionary. The independent image role retains its source object and sample body.

The focused native harness used the exact staged semantic sources and the actual prepared PDF objects. It completed in 76,291 work units with 315,656 retained bytes. All 16 focused content, page and document tests passed. The staged registered regression compares the original and saved text and samples using lopdf, compares typed objects and stream roles with the reference reader, and opens a fresh editor for the saved bytes. It awaits an integrated run.

The staged source remains outside the live package while the existing registered and mounted verification builds run. The repository source reader measured 3,980 active lines for the lift test package, 3,020 for the inference test package, and 3,063 for the snapshot-value test package.

- [Neutral mixed-resource fixture](🧫️mixed-resources.json)
- [MuPDF file and pixel oracle](📜️mixed-resource-oracle.py)
- [Native semantic probe](📜️mixed-resource-probe.py)
- [Staged resource implementation](📜️mixed-resource-stage.py)
- [Staged registered editor regression](📜️mixed-resource-editor-stage.py)
