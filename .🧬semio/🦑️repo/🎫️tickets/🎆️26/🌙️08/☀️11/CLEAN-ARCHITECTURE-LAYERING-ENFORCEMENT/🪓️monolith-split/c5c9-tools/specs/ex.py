from specs.lib import *
EX = LY + '/⚙️engine/📤️export'
H = 601
B = [969, 1277, 1909]
CH = [
    ch('📜', 'export-contract-and-state', 'export_contract_and_state', [(39, 541)],
       'Export contract and state of the layout export jobs: the tool ids, payload schemas and size caps, the export kinds and requests, the chunk rope and zip state, the export stages and close ladder, the publication and its slot.'),
    ch('🏗', 'job-admission-and-close', 'job_admission_and_close', [(543, 969)],
       'Job admission and close of the layout export job: the job record, its admission from the reserved request, the bounded close of its buffers, the close and publication steps and the publication helpers.'),
    ch('✅', 'validate-plan-and-svg', 'validate_plan_and_svg', [(971, 1280), (2146, 2232)],
       'Validate, plan and svg of the layout export job: the bounded validation of one page against the export caps, the page plan, the svg encoding and the plan primitives it shares.'),
    ch('📕', 'pdf-png-and-package-encoding', 'pdf_png_and_package_encoding', [(1282, 1915)],
       'Pdf, png and package encoding of the layout export job: the pdf objects, page and item operators, the png rows and idat chunks and the zip package entries.'),
    ch('⏭', 'advance-commit-and-codec', 'advance_commit_and_codec', [(1917, 2141), (2237, 2526)],
       'Advance, commit and codec of the layout export job: the base64 stage, the advance, the package commit, the close turn helpers, the interactive job step, the checkpoint codec, the file name helpers, the base64 decoder and the headless batch oracles.'),
]
SPEC = dict(orig='ex.rs', file=EX + '/🦀️.rs', children=CH, pre=split_impl_ln(H, B), expect_extra=split_extra_ln('ex.rs', H, B),
            external=mounted(EX + '/🦀️.rs', orig='ex.rs'))
