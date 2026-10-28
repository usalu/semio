"""🏛️ Inspect pinned I-profile declarations and original source values before new parametric geometry."""
from pathlib import Path
from html.parser import HTMLParser
import ctypes
import hashlib
import json
import sys
import urllib.request

ticket = Path(__file__).parent
dependency = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    buffer = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency, buffer, len(buffer))
    dependency = buffer.value or dependency
sys.path.insert(0, dependency)
import ifcopenshell
from ifcopenshell import ifcopenshell_wrapper


class ProfileSourceOracle:
    """🏺️ Return owned current metadata and original scalar source records only."""

    def inspect(self, path):
        """📜️ Keep all test-only external classes behind this small research interface."""
        source = ifcopenshell.open(str(path))
        declaration = ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2').declaration_by_name('IfcIShapeProfileDef')
        return dict(version=ifcopenshell.version, declarations=[dict(name=a.name(), type=str(a.type_of_attribute()), optional=a.optional()) for a in declaration.all_attributes()], profiles=[dict(id=p.id(), values=p.get_info(recursive=True)) for p in source.by_type('IfcIShapeProfileDef')])


class TextReader(HTMLParser):
    """🗞️ Extract saved primary text without executing or interpreting document instructions."""

    def __init__(self):
        """🧾️ Retain visible source fragments for a local evidence artifact."""
        super().__init__()
        self.parts = []

    def handle_data(self, data):
        """🔡️ Keep original text fragments in their source order."""
        value = data.strip()
        if value:
            self.parts.append(value)


folder = ticket / '🗑️generated/i-profile'
folder.mkdir(exist_ok=True)
url = 'https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/IfcIShapeProfileDef.htm'
data = urllib.request.urlopen(url).read()
(folder / 'IfcIShapeProfileDef.htm').write_bytes(data)
reader = TextReader()
reader.feed(data.decode('utf-8'))
(folder / 'primary.txt').write_text('\n'.join(reader.parts), encoding='utf-8')
source = Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/column-extruded-solid.ifc')
result = ProfileSourceOracle().inspect(source)
result.update(url=url, primarySha256=hashlib.sha256(data).hexdigest(), fileSha256=hashlib.sha256(source.read_bytes()).hexdigest())
(folder / 'source.json').write_text(json.dumps(result, indent=2, default=str), encoding='utf-8')
print('[DEBUG] ' + json.dumps(result, default=str))
