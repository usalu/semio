from specs.lib import *
CHD = OD + '/🔌️plugin/🪆️child'
CH = [
    ch('🧭', 'member-keys-and-paths', 'member_keys_and_paths', [(35, 135)],
       'Member keys and paths of the child members: the owned document member preparation size, the member step and path with their parsing and display.'),
    ch('🗂', 'member-registry', 'member_registry', [(137, 447)],
       'Member registry of the child members: the registry entries, admissions and registration errors and the child member registry with its drop.'),
    ch('📥', 'document-member-ingress', 'document_member_ingress', [(449, 704)],
       'Document member ingress of the child members: the owned document member ingress, its page slots, the ingress registry and their drops.'),
    ch('🔭', 'closure-source-and-content-owners', 'closure_source_and_content_owners', [(706, 831)],
       'Closure source and content owners of the child members: the document closure source view, the borrowed content owners, the retiring member lookups and the original content step owners.'),
    ch('♻', 'child-retirement-and-return-pump', 'child_retirement_and_return_pump', [(834, 1096)],
       'Child retirement and return pump of the child members: the content member metadata and retirement, the snapshot read return pump, the admission test state and the member metadata, retirement and store admission.'),
]
SPEC = dict(orig='ch.rs', file=CHD + '/🦀️.rs', children=CH, external=mounted(CHD + '/🦀️.rs', orig='ch.rs'))
