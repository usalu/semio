"""🔍 Verifies the staged PDF paint edits with the independent MuPDF reader."""
from pathlib import Path
import json
import os
import subprocess

import fitz

stage = Path(__file__).resolve().parent / "🗑️generated" / "font-size"
overlay = {Path(path).parent.name: body for path, _, body in json.loads((stage / "overlay.json").read_text(encoding="utf8"))}
case = json.loads((stage / "font-size.json").read_text(encoding="utf8"))

def function(body, name):
    start = body.index("fn " + name + "(")
    return body[start:body.index("\n}", start) + 2]

probe = r"""
#[derive(Clone, Debug, PartialEq)]
enum Op {
    EndText, BeginText, SetTextMatrix { matrix: [f64; 6] }, NextLine,
    MoveText { x: f64, y: f64 }, NextLineShowText { text: String },
    NextLineShowTextSpaced { word_spacing: f64, char_spacing: f64, text: String },
    ShowText { text: String }, ShowTextArray { items: Vec<Item> },
    SetWordSpacing { spacing: f64 }, SetCharSpacing { spacing: f64 }, SetFont { name: String, size: f64 },
}
use Op as PdfOp;
#[derive(Clone, Debug, PartialEq)]
enum Item { Codes(String), Adjust { amount: f64 } }
use Item as PdfTextArrayItem;
type Fault = String;
fn fault(reason: &str) -> Fault { reason.into() }
struct Placement { line: [f64; 6], next_origin: (f64, f64), horizontal_scale: f64 }
struct PageObject { end: usize, text_placement: Option<Placement>, font_size: f64, font: String }
fn invert([a,b,c,d,e,f]: [f64; 6]) -> Option<[f64; 6]> {
    assert_eq!([a,b,c,d], [1.0,0.0,0.0,1.0]);
    Some([a,b,c,d,-e,-f])
}
fn apply([a,b,c,d,e,f]: [f64; 6], x: f64, y: f64) -> (f64, f64) { (a*x+c*y+e,b*x+d*y+f) }
@FUNCTIONS@
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let quote = &args[0] == "quote";
    let old_size = args[1].parse::<f64>().unwrap();
    let new_size = args[2].parse::<f64>().unwrap();
    let next_x = args[3].parse::<f64>().unwrap();
    let selected = if quote {
        Op::NextLineShowTextSpaced { word_spacing: 2., char_spacing: 1., text: "E9E9".into() }
    } else {
        Op::ShowTextArray { items: vec![Item::Codes("E9".into()), Item::Adjust { amount: -50. }, Item::Codes("E9".into())] }
    };
    let tail = Op::ShowTextArray { items: vec![Item::Codes(args[4].clone()), Item::Adjust { amount: -20. }] };
    let mut content = vec![selected.clone(), tail.clone(), Op::NextLine, Op::ShowText { text: "5461696C".into() }];
    let y = if quote { 50. } else { 74. };
    let object = PageObject { end: 0, text_placement: Some(Placement {
        line:[1.,0.,0.,1.,10.,y], next_origin:(next_x,y), horizontal_scale:100.,
    }), font_size:old_size, font:"Fsrc".into() };
    restyle_text(&mut content, &object, selected.clone(), "Fsrc".into(), new_size).unwrap();
    let selected_paint = if quote { Op::ShowText { text:"E9E9".into() } } else { selected };
    assert_eq!(content.iter().filter(|op| **op == selected_paint).count(), 1);
    assert_eq!(&content[content.len()-3], &tail);
    print!("BT /Fsrc {old_size} Tf 10 74 Td 24 TL ");
    for op in content {
        match op {
            Op::NextLine => print!("T* "),
            Op::SetWordSpacing { spacing } => print!("{spacing} Tw "),
            Op::SetCharSpacing { spacing } => print!("{spacing} Tc "),
            Op::SetFont { name, size } => print!("/{name} {size} Tf "),
            Op::ShowText { text } => print!("<{text}> Tj "),
            Op::SetTextMatrix { matrix: [a,b,c,d,e,f] } => print!("{a} {b} {c} {d} {e} {f} Tm "),
            Op::ShowTextArray { items } => {
                print!("[");
                for item in items {
                    match item {
                        Item::Codes(code) => print!("<{code}> "),
                        Item::Adjust { amount } => print!("{amount} "),
                    }
                }
                print!("] TJ ");
            }
            _ => panic!("unexpected operator in typed output: {op:?}"),
        }
    }
    println!("ET");
}
"""
functions = [function(overlay["🧮️content-ops"], name) for name in (
    "text_cursor_after", "show_with_explicit_line", "restyle_text"
)] + [function(overlay["📄️document"], "append_advance")]
source = stage / "paint.rs"
source.write_text(probe.replace("@FUNCTIONS@", "\n\n".join(functions)), encoding="utf8")
binary = stage / ("paint.exe" if os.name == "nt" else "paint")
subprocess.run([
    "repo", "tools", "exec", "rustup", "--", "run", "nightly-2026-07-20",
    "rustc", "--edition=2021", "-A", "dead_code", str(source), "-o", str(binary),
], check=True)

def pdf(program):
    program = program.encode()
    rows = [
        b"<< /Type /Catalog /Pages 2 0 R >>",
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>",
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 180 100] /Resources << /Font << /Fsrc 5 0 R >> >> /Contents 4 0 R >>",
        f"<< /Length {len(program)} >>\nstream\n".encode() + program + b"\nendstream",
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>",
    ]
    output = bytearray(b"%PDF-1.7\n")
    xref = [b"0000000000 65535 f \n"]
    for number, row in enumerate(rows, 1):
        xref.append(f"{len(output):010d} 00000 n \n".encode())
        output.extend(f"{number} 0 obj\n".encode() + row + b"\nendobj\n")
    start = len(output)
    output.extend(f"xref\n0 {len(xref)}\n".encode() + b"".join(xref)
                  + f"trailer\n<< /Root 1 0 R /Size {len(xref)} >>\nstartxref\n{start}\n%%EOF\n".encode())
    return output

def read(program):
    with fitz.open(stream=pdf(program), filetype="pdf") as doc:
        rows = [(c["c"], s["size"], c["origin"])
                for b in doc[0].get_text("rawdict")["blocks"] for l in b["lines"]
                for s in l["spans"] for c in s["chars"] if c["c"].strip()]
        assert "".join(c for c, _, _ in rows) == "éé" + case["sameLine"] + case["followingLine"], rows
        return rows

for kind, selected in [("array", "[<E9> -50 <E9>] TJ"), ("quote", '2 1 <E9E9> "')]:
    program = f'BT /Fsrc {case["before"]} Tf 10 74 Td 24 TL {selected} [<{case["sameLineCode"]}> -20] TJ T* (Tail) Tj ET'
    before = read(program)
    after_program = subprocess.check_output([
        str(binary), kind, str(case["before"]), str(case["after"]),
        str(before[2][2][0]), case["sameLineCode"],
    ], text=True)
    after = read(after_program)
    assert after[0][1] == after[1][1] == case["after"], after
    assert after[0][2] == before[0][2], after
    for actual, expected in zip(after[2:], before[2:], strict=True):
        assert actual[:2] == expected[:2] and all(abs(a-e) < 0.0001 for a,e in zip(actual[2],expected[2])), (actual, expected)
    print(f"[DEBUG] MuPDF verified {kind} selected paint and all following glyph origins and font sizes")
