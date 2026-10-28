"""🧩️ Confirm complete original source allows distinct products sharing a one-item definition shape."""
from pathlib import Path
import ctypes
import json
import sys

ticket = Path(__file__).parent
dependency = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)
import ifcopenshell
import ifcopenshell.validate
from ifcopenshell import ifcopenshell_wrapper

schema = ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
items = next(a for a in schema.declaration_by_name('IfcRepresentation').all_attributes() if a.name() == 'Items')
folder = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official')
rows = []
for name, product_id, copies in [('csg-primitive.ifc', 1000, 2), ('polygonal-face-tessellation.ifc', 30, 1)]:
    source = ifcopenshell.open(str(folder / name))
    product = source.by_id(product_id)
    for at in range(copies):
        values = list(product)
        values[0] = '000000000000000000000' + str(at + 3)
        source.create_entity(product.is_a(), *values)
    logger = ifcopenshell.validate.json_logger()
    ifcopenshell.validate.validate(source, logger, express_rules=True)
    rows.append(dict(file=name, products=copies+1, oneSharedShape=product.Representation.id(), diagnostics=logger.statements))
result = dict(version=ifcopenshell.version, itemsDeclaration=str(items.type_of_attribute()), rows=rows)
(ticket / '🗑️generated/shared-representation-oracle.json').write_text(json.dumps(result, indent=2, default=str), encoding='utf-8')
print('[DEBUG] ' + json.dumps(result, default=str))
