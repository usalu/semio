from specs.lib import *
JR = M + '/🧵️job'
PRE = close_regions({}, ['//#endregion 🧪️Tests'])
CH = [
    ch('🕰', 'clock-identity-and-budget', 'clock_identity_and_budget', [(41, 197)],
       'Clock, identity and budget: the synchronous poll, the cancel tokens, the millisecond and microsecond clocks, the revision and operation identities, the commit validation and the step budget with its lane constants.'),
    ch('📄', 'payload-pages-and-ledger', 'payload_pages_and_ledger', [(201, 452)],
       'Payload pages and ledger: the payload size constants, the payload stream and admission fault, the page source, the rejected page, the operation ledger and the retained job payload with its reader.'),
    ch('✍', 'payload-writer-and-grant', 'payload_writer_and_grant', [(454, 675)],
       'Payload writer and grant: the retained job payload writer with its drop and the payload page grant with its drop.'),
    ch('🚦', 'clock-stride-and-job-protocol', 'clock_stride_and_job_protocol', [(1033, 1220)],
       'Clock stride and job protocol: the clock stride, the checkpoint, commit candidate, fault and step outcome records, the close step, the thread transfer traits and the interactive job trait.'),
]
SPEC = dict(orig='jr.rs', file=JR + '/🦀️.rs', children=CH, wiring='block', external=mounted(JR + '/🦀️.rs', orig='jr.rs'),
            pre=PRE, region_fix=1, crate_files=[])
