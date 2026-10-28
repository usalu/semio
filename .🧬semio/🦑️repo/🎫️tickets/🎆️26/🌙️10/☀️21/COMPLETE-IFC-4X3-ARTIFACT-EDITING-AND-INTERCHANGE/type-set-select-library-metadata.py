"""🧭️ Observe released SET metadata through an owned external schema interface."""
from pathlib import Path
import ctypes,json,sys
ticket=Path(__file__).parent
dependency=str(ticket/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(dependency,buffer,len(buffer))
    dependency=buffer.value or dependency
sys.path.insert(0,dependency)

class ReleasedSetMetadata:
    """🧊️ Return plain metadata without exporting the external library's type objects."""
    def observe(self):
        import ifcopenshell
        schema=ifcopenshell.ifcopenshell_wrapper.schema_by_name('IFC4X3_ADD2')
        declaration=schema.declaration_by_name('IfcPropertySetDefinitionSet')
        aggregate=declaration.declared_type()
        return {'oracle':'IfcOpenShell','version':ifcopenshell.version,'schema':'IFC4X3_ADD2','name':declaration.name(),'kind':aggregate.type_of_aggregation_string(),'lower':aggregate.bound1(),'upper':aggregate.bound2(),'element':str(aggregate.type_of_element())}

output=ReleasedSetMetadata().observe()
(ticket/'🗑️generated/type-set-select-library-metadata.json').write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG]',json.dumps(output))
