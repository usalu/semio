"""🌡️ Check roof U-values and their provenance in both IFC generations."""
from pathlib import Path
import importlib.util,json,sys
g=Path(__file__).resolve().parent/'🗑️generated'
p=next((Path.cwd()/'✏️s/🔌️plugins/🏙️bim').rglob('🔥️export-bim-1-ifc-energy/🐍️.py'))
loader=importlib.util.spec_from_file_location('bim_ifc_energy_oracle',p)
module=importlib.util.module_from_spec(loader)
loader.loader.exec_module(module)
root=next((Path.cwd()/'✏️s/🔌️plugins/🏙️bim').rglob('🧫️fixtures/🏗️ifc/🔥️energy'))
for version in ['2x3','4']:
    for authored in (False,True):
        model=json.loads((root/'📸️snapshot/🔣️.json' if authored else g/'thermal-unoverridden.json').read_text())
        path=root/('energy-'+version+'.ifc') if authored else g/'exchange-wasi'/('unoverridden-'+version+'.ifc')
        table,problems=module.audit(path,model)
        assert not problems,problems
        prop=table['elements']['rf-main']['Pset_RoofCommon'].get('ThermalTransmittance')
        derived='Pset_RoofCommon.ThermalTransmittance' in table['derived'].get('rf-main','').split(',')
        if authored:
            assert prop==['IFCREAL',0.12] or prop==('IFCREAL',0.12),prop
            assert not derived
        elif version=='4':
            assert prop[0]=='IFCTHERMALTRANSMITTANCEMEASURE' and prop[1]>0 and derived,(prop,derived)
        else:
            assert prop is None and not derived,(prop,derived)
        print(version,'authored' if authored else 'computed',prop,'derived',derived,flush=True)
