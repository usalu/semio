---
name: Fix empty TOC rows
overview: "Fix two independent bugs causing all Semio Table/TOC/register rows to render empty: a missing exp_args:Vxx expansion that stores symbolic variable references instead of literal row values, and a build pipeline that only guarantees a second compile pass for documents using Panels."
todos: []
isProject: false
---

## Root cause 1: row content corruption in materialize (primary bug, affects all tables)

In [print/tex/semio-window.sty](print/tex/semio-window.sty):

```61:73:print/tex/semio-window.sty
\cs_new_protected:Npn \semio_window_register_body_add:Vxx #1#2#3 {
  \tl_put_right:Nn \l_tmpa_tl { \semio@register@data {#1} {#2} {#3} }
}

\cs_new_protected:Npn \semio_window_register_body_materialize:n #1 {
  \tl_clear:N \l_tmpa_tl
  \int_step_inline:nn { \seq_count:c { g_semio_register_ #1 _num_seq } } {
    \tl_set:Nx \l_tmpb_tl { \seq_item:cn { g_semio_register_ #1 _num_seq } { ##1 } }
    \tl_set:Nx \l_tmpc_tl { \seq_item:cn { g_semio_register_ #1 _title_seq } { ##1 } }
    \str_set:Nx \l_tmpd_str { \seq_item:cn { g_semio_register_ #1 _label_seq } { ##1 } }
    \semio_window_register_body_add:Vxx \l_tmpb_tl \l_tmpc_tl \l_tmpd_str
  }
  \exp_args:No \cs_gset_nopar:cpn { semio@register@body@#1 } { \tl_use:N \l_tmpa_tl }
}
```

`\semio_window_register_body_add:Vxx` is named as if it receives V/x/x-expanded arguments, but it is defined via plain `\cs_new_protected:Npn` and called directly (line 71) without the `\exp_args:Vxx` wrapper that would actually perform that expansion. So `#1`/`#2`/`#3` inside it are the literal control-sequence tokens `\l_tmpb_tl`, `\l_tmpc_tl`, `\l_tmpd_str`, not their string values. Since `\tl_put_right:Nn` does not expand its argument, every accumulated row is the same symbolic reference `\semio@register@data{\l_tmpb_tl}{\l_tmpc_tl}{\l_tmpd_str}`, repeated for each of the N entries. These shared expl3/tcolorbox scratch registers get overwritten by unrelated code (opening the `Table` window/tcolorbox) before the table body is actually typeset, so every row's cell content evaluates to empty.

This function backs every register table (`\maketableofcontents` and all `\listof*` commands), explaining why all tables are affected.

Confirmed via evidence: `dist/zwischenbericht.sctoc` has 109 valid entries (tracking works); extracted PDF text shows page 2 (Inhaltsverzeichnis) contains only the 3 header words and zero row content, while page 3 jumps straight into body content.

Fix: change the call site to

```latex
\exp_args:Vxx \semio_window_register_body_add:Vxx \l_tmpb_tl \l_tmpc_tl \l_tmpd_str
```

so literal values are baked into `\l_tmpa_tl` at accumulation time.

## Root cause 2: no guaranteed second compile pass without Panels

In [print/script.ts](print/script.ts):

```309:326:print/script.ts
async function compilePrintDocumentWithPanels(tectonic: string, texAbs: string, outDir: string): Promise<void> {
	const workDir = dirname(texAbs);
	const jobname = basename(texAbs, ".tex");
	resetPanelArtifacts(workDir, outDir, jobname);
	compilePrintDocument(tectonic, texAbs, outDir);
	const manifestPath = panelManifestPath(outDir, jobname);
	if (!existsSync(manifestPath)) return;
	const entries = parsePanelManifest(manifestPath);
	if (entries.length === 0) return;
	const pdfPath = join(outDir, `${jobname}.pdf`);
	await renderPanelGlass({ ... });
	compilePrintDocument(tectonic, texAbs, outDir);
}
```

`mit-bestand/bericht/zwischenbericht/zwischenbericht.tex` has no `\begin{Panel}`, so this returns after exactly one `compilePrintDocument` call. `\maketableofcontents` reads `.sctoc` at the top of the document, but tracking writes happen later in that same pass as headings are processed — too late for that pass's own read. Confirmed via build log: only one `Document Class` load and one `Output written` message (genuinely one pass). The render can only ever see a stale leftover `.sctoc` from a separate prior invocation (empty on a clean build).

Fix: make the second `compilePrintDocument` call unconditional in `compilePrintDocumentWithPanels`, independent of whether a panel manifest exists, so every template gets a guaranteed 2-pass compile (panel glass rendering still only runs when a manifest with entries exists).

## Execution steps

1. Reopen ticket `26/07/08/TOC-SEMIO-WINDOW-TABLES` per repo convention.
2. Apply the one-line fix in `print/tex/semio-window.sty` (line 71).
3. Restructure `compilePrintDocumentWithPanels` in `print/script.ts` for an unconditional second pass.
4. Rebuild `mit-bestand/bericht` zwischenbericht (light + dark) via `bun ./📜️script.ts build`.
5. Run `bun print/script.ts test` to rebuild all templates (paper, report, forschungsbericht, kompaktbericht, flyer) and confirm no regressions.
6. Verify via PDF text extraction (pdfjs) that TOC pages contain actual row text (e.g. "Ergebnisse", "Forschungsfragen und Vorgehen", nested paragraph numbers) with correct dot-count indentation, and that other `\listof*` registers populate where used.
7. Update `.repo/🎫️/26/07/08/TOC-SEMIO-WINDOW-TABLES/verify-log.md` with findings and close the ticket.</plan>
   <todos>
   <todo id="fix-vxx" content="Fix missing \exp_args:Vxx at semio_window_register_body_add call site in print/tex/semio-window.sty"/>
   <todo id="fix-second-pass" content="Make the second compile pass in compilePrintDocumentWithPanels (print/script.ts) unconditional, not gated on Panel manifest"/>
   <todo id="rebuild-verify" content="Rebuild mit-bestand zwischenbericht and print templates, verify PDF TOC/register rows via text extraction"/>
   <todo id="close-ticket" content="Update verify-log.md and close ticket 26/07/08/TOC-SEMIO-WINDOW-TABLES"/>
   </todos>
