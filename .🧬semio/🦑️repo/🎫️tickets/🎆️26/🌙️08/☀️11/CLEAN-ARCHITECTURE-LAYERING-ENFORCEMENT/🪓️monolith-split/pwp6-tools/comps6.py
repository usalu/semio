import sys; sys.path.insert(0, '.')
from lib6 import *
from fields6 import parse_fields, comp_of

# composer field -> (struct name, crate, plan component key)
COMPONENTS = collections.OrderedDict([
 ('constructor', ('ConstructorState', 'instance-stores', 'constructor')),
 ('stores', ('InstanceStores', 'instance-stores', 'stores')),
 ('roster', ('PeerRoster', 'instance-stores', 'roster')),
 ('ingress', ('ActionIngress', 'instance-stores', 'ingress')),
 ('interaction', ('LocalInteraction', 'instance-stores', 'interaction')),
 ('history', ('CommandHistory', 'instance-history', 'history')),
 ('host_events', ('HostEvents', 'instance-history', 'hostevent')),
 ('txn', ('Transactions', 'instance-history', 'txn')),
 ('child_composition', ('ChildComposition', 'instance-children', 'children')),
 ('pending', ('PendingReserved', 'instance-jobs', 'pending')),
 ('tools', ('ToolOperations', 'instance-jobs', 'tools')),
 ('media', ('MediaTransfers', 'instance-docs', 'media')),
 ('ingest', ('EnvelopeJobs', 'instance-docs', 'ingest')),
 ('maintenance', ('MaintenanceCursors', 'instance-ladders', 'maintenance')),
 ('close', ('CloseCursors', 'instance-ladders', 'close')),
])
PLAN2FIELD = {v[2]: k for k, v in COMPONENTS.items()}
COMPOSER_KEEP = {'core', 'toolmachine', 'timetravel', 'toolrun'}      # app, mounted_policy, tool_machines, time_travel, tool_runs stay composer fields
COMP_RANK = ['core', 'constructor', 'stores', 'roster', 'ingress', 'interaction', 'history', 'hostevent', 'txn', 'children', 'pending', 'tools', 'media', 'ingest', 'toolmachine', 'timetravel', 'toolrun', 'maintenance', 'close']
BOUND = {'A': 'A: ArtifactApp', 'M': 'M: SpaceMember + MemberFactory'}

def field_table():
    """name -> dict(comp plan key, composer field or None)"""
    t = {}
    for f in parse_fields():
        c = comp_of(f['name'])
        t[f['name']] = dict(plan=c, cf=PLAN2FIELD.get(c), vis=f['vis'], f=f)
    return t

def generics_of(field_decls):
    txt = '\n'.join(field_decls)
    g = []
    if re.search(r'\bA\b', txt): g.append('A')
    if re.search(r'\bM\b', txt): g.append('M')
    return g
