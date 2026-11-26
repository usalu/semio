"""📄 Verify the real release entry contract against a separate HTML fixture."""
from pathlib import Path
import runpy
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
tools = runpy.run_path(str(ticket / "accept-site.py"))
generated = tools["GENERATED"]
generated.mkdir(exist_ok=True)
verify = tools["verify_site_entry"]

with TemporaryDirectory(dir=generated, prefix="ep-") as scratch:
    site = Path(scratch)
    host = "demonstrator.entwerfen.mit-bestand.de"
    title = "Designing with Existing Stock · Demonstrator · Entwerfen mit Bestand · Demonstrator"
    cname = site / "CNAME"
    index = site / "index.html"
    nojekyll = site / ".nojekyll"
    html = f'<title>{title}</title><div id="root"></div><script type="module" src="./assets/app.js"></script>'
    (site / "assets").mkdir()
    (site / "assets/app.js").touch()
    cname.write_text(host + "\n", encoding="utf-8")
    index.write_text(html, encoding="utf-8")
    nojekyll.write_bytes(b"")
    verify(site)
    for path, contents in ((cname, host + "\nother.example\n"), (nojekyll, "present but not empty"),
                           (index, html.replace(title, "another app")), (index, html.replace('id="root"', 'id="other"')),
                           (index, html.replace("app.js", "missing.js"))):
        before = path.read_bytes()
        path.write_text(contents, encoding="utf-8")
        try:
            verify(site)
        except ValueError:
            pass
        else:
            raise AssertionError(f"Invalid release entry accepted: {path.name}")
        path.write_bytes(before)

print("Release entry fixture matched the real CNAME, .nojekyll and built HTML contract.")
