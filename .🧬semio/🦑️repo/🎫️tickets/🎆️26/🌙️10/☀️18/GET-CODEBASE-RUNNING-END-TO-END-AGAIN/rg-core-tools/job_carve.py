import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
import rgl
R = rgl.ROOT
J = R + "/🧰️framework/🔨️modules/🧵️job"
N = J + "/📬️outcome/📦️builder/📦️packages/🦀️rust"
FILES = {
 "pages": J + "/📄️payload-pages-and-ledger/🦀️.rs",
 "writer": J + "/✍️payload-writer-and-grant/🦀️.rs",
 "staged_writer": J + "/📬️outcome/✍️writer/⛽️staged/🦀️.rs",
 "source": J + "/📬️outcome/📖️source/🦀️.rs",
 "builder": J + "/📬️outcome/📦️builder/🦀️.rs",
 "builder_staged": J + "/📬️outcome/📦️builder/⛽️staged/🦀️.rs",
}
STEP = {
 r"cx\s*:\s*&mut\s+StepContext<'_>": "cx:&mut impl JobPayloadStep",
 r"cx\s*:\s*&\s*StepContext<'_>": "cx:&impl JobPayloadStep",
}
for k, p in FILES.items():
    t = rgl.read(p)
    for rx, rep in STEP.items():
        t = re.sub(rx, rep, t)
    t = re.sub(r'&cx\.payload_ledger\b(?!\()', 'cx.payload_ledger()', t)
    t = re.sub(r'\bcx\.payload_ledger\b(?!\()', 'cx.payload_ledger()', t)
    t = re.sub(r'cx\.payload_page_granted\s*=\s*true', 'cx.grant_payload_page()', t)
    t = re.sub(r'\bcx\.payload_page_granted\b(?!\()', 'cx.payload_page_granted()', t)
    rgl.write(p, t)
    print(k, t.count("StepContext"))
