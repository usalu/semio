"""📕️ Validates written PDFs with two independent readers: PyMuPDF (open, page count, fonts, text,
render ink) and poppler's pdftotext. Usage: python3 🐍️validate-pdfs.py <file.pdf>... [--password pw]"""
import subprocess, sys, json
import fitz
files = [a for a in sys.argv[1:] if not a.startswith("--")]
password = sys.argv[sys.argv.index("--password") + 1] if "--password" in sys.argv else ""
for path in files:
    report = {"path": path.split("/")[-1]}
    try:
        doc = fitz.open(path)
        if doc.needs_pass:
            report["encrypted"] = True
            report["authenticated"] = bool(doc.authenticate(password))
        report["pages"] = doc.page_count
        report["fonts"] = sorted({f[3] for p in doc for f in p.get_fonts()})
        report["text"] = [p.get_text().strip()[:80] for p in list(doc)[:2]]
        report["annots"] = [a.type[1] for a in doc[0].annots()] if doc.page_count else []
        report["toc"] = doc.get_toc()[:3]
        report["metadata"] = {k: v for k, v in doc.metadata.items() if v}
        report["embedded"] = doc.embfile_count()
        pix = doc[0].get_pixmap(dpi=36, alpha=False)
        samples = pix.samples
        ink = sum(1 for i in range(0, len(samples), pix.n) if any(samples[i + c] < 250 for c in range(min(3, pix.n))))
        report["ink_px"] = ink
    except Exception as error:
        report["error"] = repr(error)
    try:
        out = subprocess.run(["pdftotext", "-l", "1"] + (["-upw", password] if password else []) + [path, "-"], capture_output=True, text=True, timeout=60)
        report["pdftotext"] = out.stdout.strip()[:80] if out.returncode == 0 else f"exit {out.returncode}: {out.stderr.strip()[:120]}"
    except Exception as error:
        report["pdftotext"] = repr(error)
    print(json.dumps(report, ensure_ascii=False))
