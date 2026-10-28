"""🔮 Independently validate unchanged official ADD2 models with IfcOpenShell."""
import ctypes
import json
import pathlib
import subprocess
import sys
import time
ticket = pathlib.Path(__file__).resolve().parent
module_path = str(ticket / '🗑️generated/oracle')
if sys.platform == 'win32':
    short = ctypes.create_unicode_buffer(32768)
    ctypes.windll.kernel32.GetShortPathNameW(module_path, short, len(short))
    module_path = short.value or module_path
output = ticket / '🗑️generated/corpus'
manifest = json.loads((output / 'downloaded.json').read_text(encoding='utf-8'))

def validate(source):
    sys.path.insert(0, module_path)
    import ifcopenshell
    import ifcopenshell.validate
    row = {'path': source['path'], 'sha256': source['sha256']}
    start = time.monotonic()
    try:
        model = ifcopenshell.file.from_string((output / source['file']).read_text(encoding='utf-8'))
        row['entities'] = len(list(model))
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(model, logger, express_rules=True)
        row['diagnostics'] = [{key: value.id() if isinstance(value, ifcopenshell.entity_instance) else str(value) for key, value in message.items()} for message in logger.statements]
    except Exception as error:
        row['error'] = str(error)
    row['ms'] = round((time.monotonic() - start) * 1000)
    row['version'] = ifcopenshell.version
    return row

if len(sys.argv) == 3 and sys.argv[1] == '--model':
    print('RESULT ' + json.dumps(validate(manifest['files'][int(sys.argv[2])])) )
else:
    rows = []
    for index, source in enumerate(manifest['files']):
        if source['schema'] != 'IFC4X3_ADD2':
            continue
        start = time.monotonic()
        try:
            process = subprocess.Popen([sys.executable, __file__, '--model', str(index)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding='utf-8', errors='replace')
            try:
                stdout, stderr = process.communicate(timeout=20)
            except BaseException:
                process.kill()
                process.communicate()
                raise
            results = [line[7:] for line in stdout.splitlines() if line.startswith('RESULT ')]
            row = json.loads(results[-1]) if results else {'path': source['path'], 'sha256': source['sha256'], 'error': f'Oracle process {process.returncode}: {stderr[-1000:]}'}
        except subprocess.TimeoutExpired:
            row = {'path': source['path'], 'sha256': source['sha256'], 'error': 'Independent oracle exceeded 20-second per-model bound', 'timeout': True}
        row['ms'] = round((time.monotonic() - start) * 1000)
        rows.append(row)
        (output / 'ifcopenshell.json').write_text(json.dumps({'commit': manifest['commit'], 'implementation': 'IfcOpenShell', 'files': rows}, indent=2) + '\n', encoding='utf-8')
        print('[DEBUG] ' + json.dumps({'completed': len(rows), 'total': len(manifest['files']), 'path': row['path'], 'diagnostics': len(row.get('diagnostics', [])), 'error': row.get('error'), 'ms': row['ms']}), flush=True)
