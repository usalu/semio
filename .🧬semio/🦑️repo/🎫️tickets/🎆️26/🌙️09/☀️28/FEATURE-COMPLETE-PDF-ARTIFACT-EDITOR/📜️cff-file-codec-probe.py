"""🔡️ Uses a captured PDF font program with the native font codec."""

import importlib.util
from pathlib import Path
import re
import sys

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated"
source = ticket / "📜️xref-section-tests.py"
spec = importlib.util.spec_from_file_location("graph_probes", ticket / "🎞️file-graph-tests.py")
probes = importlib.util.module_from_spec(spec)
sys.dont_write_bytecode = True
spec.loader.exec_module(probes)

def exercise(base, pdf, body=None, staged=None):
    base = Path(base).resolve()
    code = source.read_text()
    code = "\n".join(line for line in code.splitlines() if "mod codec;" not in line and "pub use codec::*;" not in line) + "\n"
    code = code.replace('shutil.copyfile(t/"📜️flate-run-tests.py",runner)', 'copy_native_runner(t, runner)')
    code = code.replace('font_overlay=(generated/"cff-integration/🔨️modules/🔤️fonts") if cff_pdf else font_modules', 'font_overlay=font_modules')
    for reference in ("@CONTENT_CONTRACT@", "@PAGE_CONTRACT@"):
        code = "\n".join(line for line in code.splitlines() if not line.startswith(f'#[path = "{reference}"]')) + "\n"
    original = "#[test]\nfn imported_opentype_codec_uses_declared_byte_codes_and_cubic_glyphs()"
    if body is None:
        code = code.replace(original, "fn main()")
    else:
        code, replacements = re.subn(re.escape(original) + r" \{\n.*?^}", lambda _: "fn main() {\n" + body.rstrip(), code, count=1, flags=re.MULTILINE | re.DOTALL)
        assert replacements == 1
    if staged:
        code = code.replace('font_codec=(font_overlay/"🦀️.rs").read_text()', 'font_codec=staged.joinpath("codec.rs").read_text()')
        code = code.replace('(font_overlay/"✒️cff/🦀️.rs").resolve()', 'staged.joinpath("cff.rs")')
    code = code.replace("testkit = (l.parents[1]", "extra = stage_paths(base, out, extra + filters_owner)\ntestkit = (l.parents[1]")
    engine = base / "🔨️modules/🗜️filters/📦️packages/🦀️rust/🦀️.rs"
    filters_owner = f'#[path = "{engine.as_posix()}"] mod filters_native;\npub use filters_native::*;\n'
    sys.argv = [str(source), str(base / "🔨️modules/🔤️lexer"), str(ticket), str(base / "🔨️modules/🗜️filters"),
                str(output), str(base / "🔨️modules/🔗️xref"), str(output / "large-mounted-pdf.pdf"), str(output / pdf)]
    exec(compile(code, str(source), "exec"),
         {"__file__": str(source), "copy_native_runner": probes.copy_native_runner, "stage_paths": probes.stage_paths,
          "filters_owner": filters_owner, "staged": staged})

if __name__ == "__main__":
    exercise(sys.argv[1], "cff-opentype.pdf")
