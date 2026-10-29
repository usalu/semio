# Roof Drawing Retrieval R47

Read-only bounded retrieval audit, 2026-10-29. Parent owns ticket lifecycle and generated cleanup. No model, test, AGENTS, Git, or worktree change.

## Verified Repository Provenance

The official [UVaDOC item 10324/66393](https://uvadoc.uva.es/handle/10324/66393) is titled *Savoye as built. Anejos planimétricos*, authored by Fernando Zaparaín Hernández, published by Ediciones Universidad de Valladolid in 2024. The indexed metadata describes plans of the villa at completion in 1930. This is a modern scholarly reconstruction of the 1930 state, not an original FLC drawing. The repository lists updatedVersion, openAccess and CC BY-NC-ND 4.0.

The roof attachment is listed as SAVOYE PLANTA+2.pdf, 196.9 KB, described as the roof plan. Adjacent listed sheets include basement, ground and first floor plans, plus NE/NO/SE/SO elevations and sections. The live attachment href was not recovered. No PDF bytes, checksum, scale, coordinates, dimensions or newly registered architectural facts were obtained.

## Attempt Ledger

1. Web search of exact official handle and exact quoted attachment filename succeeded in returning indexed official item metadata. The main, German and Portuguese locale variants all describe the same item. Search results did not expose any attachment link.
2. Web open of the official item, HTTP variant, German locale variant and show=full variant returned fetch timeouts. The cached search-result reference also opened as a timeout. mode=full and locale-attribute=es with show=full were inaccessible to the web tool.
3. Direct Node fetch of HTTPS item, HTTP item and public OAI GetRecord with metadataPrefix=mets returned fetch failures. OAI identifier was oai:uvadoc.uva.es:10324/66393. These are attempted standard DSpace metadata routes; no successful response established their support.
4. Direct Node fetch of mode=full, OAI GetRecord metadataPrefix=ore, and metadata/handle/10324/66393/mets.xml each timed out at 10 seconds. Web requests to OAI/METS routes were inaccessible. No metadata payload was available.
5. Exact filename and bitstream/10324/66393 search queries returned only the official item and book; no indexed small attachment URL.
6. One bounded test of the conventional named bitstream/handle URL and one sequence-4 conventional DSpace bitstream URL returned connection timeouts. The latter uses the fourth position in the displayed attachment list but is **unverified** and must not be cited as a recovered attachment URL. Exact attempted URLs are retained in roof-drawing-retrieval-r47.mjs.

The final direct fetch failure identifies ConnectTimeoutError to uvadoc.uva.es:443 after 10 seconds, rather than a PDF size problem. No full-book transfer, Cloudflare bypass, source geometry modification, PDFJS worker or operator list was attempted.

## Disposition

The official small roof sheet exists according to the repository index, but this environment could not establish a connection to the repository or recover its attachment href. Do not treat a guessed DSpace bitstream route as verified provenance. The earlier author-hosted annex sheets remain the available local reconstruction evidence; no new plan registration is supplied by R47.
