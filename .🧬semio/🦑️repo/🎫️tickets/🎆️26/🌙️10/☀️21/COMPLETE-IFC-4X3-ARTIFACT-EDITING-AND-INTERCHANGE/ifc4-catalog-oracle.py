"""🔬 Compare all IFC4 ADD2 TC1 declaration domains against independent IfcOpenShell."""
import hashlib
import json
import pathlib
import sys

ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    import ctypes
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
sys.path.insert(0, module_path)
import ifcopenshell
from ifcopenshell import ifcopenshell_wrapper as wrapper

root = ticket.parents[6]
base = root / '✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📚️catalog'
catalog = json.loads((base / '🔣️.json').read_text(encoding='utf-8'))
schema = wrapper.schema_by_name('IFC4')
checks = 0
failures = []


def equal(actual, expected, context):
    global checks
    checks += 1
    if actual != expected:
        failures.append(dict(context=context, actual=actual, expected=expected))


def domain(reference, expected, context):
    if expected['kind'] == 'aggregate':
        aggregate = reference.as_aggregation_type()
        equal(aggregate is not None, True, context + ' aggregate')
        if aggregate is None:
            return
        equal(aggregate.type_of_aggregation_string().upper(), expected['aggregate'], context + ' aggregate kind')
        for bound, method in [('lower', aggregate.bound1), ('upper', aggregate.bound2)]:
            if expected[bound] is not None:
                equal(method(), expected[bound], context + ' ' + bound)
        domain(aggregate.type_of_element(), expected['item'], context + ' item')
        return
    named = reference.as_named_type()
    if named:
        equal(named.declared_type().name().upper(), expected['name'], context + ' named type')
        return
    simple = reference.as_simple_type()
    equal(simple is not None, True, context + ' primitive')
    if simple is not None:
        builtin = next((name.upper() for name in ['binary', 'boolean', 'integer', 'logical', 'number', 'real', 'string'] if simple.declared_type() == getattr(wrapper.simple_type, name + '_type')), None)
        equal(builtin, expected['name'], context + ' primitive type')


equal(hashlib.sha256((base / 'IFC4.exp').read_bytes()).hexdigest(), catalog['source']['sha256'], 'source checksum')
equal(len(schema.entities()), len(catalog['entities']), 'all entity count')
for name, declaration in catalog['entities'].items():
    reference = schema.declaration_by_name(name)
    equal(reference.is_abstract(), declaration['abstractEntity'], name + ' abstract')
    parent = reference.supertype()
    equal(parent.name().upper() if parent else None, declaration['supertype'], name + ' supertype')
    attrs = reference.all_attributes()
    equal([attr.name() for attr in attrs], [attr['name'] for attr in declaration['allAttributes']], name + ' serialization order')
    equal([attr.optional() for attr in attrs], [attr['optional'] for attr in declaration['allAttributes']], name + ' optionality')
    equal(list(reference.derived()), [attr['derived'] for attr in declaration['allAttributes']], name + ' derived positions')
    for attr, own in zip(attrs, declaration['allAttributes']):
        domain(attr.type_of_attribute(), own['typeExpr'], name + '.' + own['name'])
for name, declaration in catalog['types'].items():
    reference = schema.declaration_by_name(name)
    if declaration['kind'] == 'enum':
        equal([item.upper() for item in reference.enumeration_items()], declaration['members'], name + ' enum members')
    elif declaration['kind'] == 'select':
        equal([item.name().upper() for item in reference.select_list()], declaration['members'], name + ' select members')
    else:
        domain(reference.declared_type(), declaration['typeExpr'], name + ' defined domain')
result = dict(implementation='IfcOpenShell', version=ifcopenshell.version, checks=checks, failures=failures)
(ticket / '🗑️generated/catalog4-oracle.json').write_text(json.dumps(result, indent=2), encoding='utf-8')
print(json.dumps(result))
if failures:
    raise SystemExit(1)
