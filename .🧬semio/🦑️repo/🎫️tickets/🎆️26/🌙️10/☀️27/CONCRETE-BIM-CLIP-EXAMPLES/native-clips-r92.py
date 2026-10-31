"""🎬 Execute focused current-source native BIM gates with retained provenance."""
import hashlib
import importlib
import json
import os
from pathlib import Path
import sys
import time
import threading
import _thread
import unittest

ROOT = next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True
TICKET = Path(__file__).parent
OUTPUT = TICKET / '🗑️generated' / 'native-r92'
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ.update(SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT), SEMIO_TEST_MANIM='1', PYTHONUTF8='1', SEMIO_TEACHING_LANGUAGE='en')
CLIP = ROOT / '🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
MODE = sys.argv[1] if len(sys.argv) > 1 else 'gates'
if MODE in ('all','diagnostic','current','discover'):
    if len(sys.argv)>2:raise ValueError('A complete native cohort cannot use a topic filter')
    os.environ.pop('SEMIO_TEST_BIM_TOPICS',None)
elif len(sys.argv) > 2:
    os.environ['SEMIO_TEST_BIM_TOPICS'] = sys.argv[2]

def hashes():
    """🔐 Capture authored closure, bundled fonts, model fixtures and owned native runner."""
    runtime = importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
    theme = importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️')
    paths = {p for p in CLIP.rglob('*') if p.suffix in {'.py', '.json', '.mjs', '.cjs', '.md'} and '__pycache__' not in p.parts}
    for folder in ('📊️information', '🧬️objects', '👁️representations', '🧩️models', '🔌️exchange', '🔗️consistency'):
        paths.update(runtime.source_dependencies(CLIP / folder / '🐍️.py'))
    paths.update(theme.source_files())
    paths.update((Path(__file__), Path(runtime.__file__),
                  ROOT / '🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml',
                  ROOT / '🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock'))
    return {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths) if p.is_file()}

def relative_file(filename):
    """🗂️ Normalize native module identities without filesystem traversal."""
    try:
        path = Path(os.path.abspath(filename)).relative_to(ROOT)
        return None if any(part in {'.venv', 'node_modules', 'site-packages', '__pycache__'} for part in path.parts) else path.as_posix()
    except (ValueError, TypeError):
        return None


def canonical(test):
    """🪪 Keep unique repository identities beside native unittest IDs."""
    function = getattr(test, '_testFunc', None)
    if function is not None:
        defaults = getattr(function, '__defaults__', ()) or ()
        if function.__name__ == '<lambda>' and defaults and hasattr(defaults[0], '__name__'):
            source = defaults[0]
            file = relative_file(getattr(source, '__file__', None))
            return (file[:-3].replace('/', '.') if file else source.__name__) + '.run'
        file = relative_file(function.__code__.co_filename)
        return (file[:-3].replace('/', '.') if file else function.__module__) + '.' + function.__qualname__
    parent = getattr(test, 'test_case', None)
    if parent is not None:
        return canonical(parent) + ' ' + str(test.params)
    module = sys.modules.get(type(test).__module__)
    file = relative_file(getattr(module, '__file__', None))
    method = getattr(test, '_testMethodName', None)
    return file[:-3].replace('/', '.') + '.' + type(test).__qualname__ + '.' + method if file and method else test.id()


CODE_CACHE = {}
CODE_RECORDS = {}
FRESH_CACHE = {}
OBSERVER_ERRORS = []


def constant(value):
    """🧮 Encode compiler constants without marshal reference bookkeeping or numeric coercion."""
    import types
    import struct
    if isinstance(value, types.CodeType):
        return ['code', code_hash(value)]
    if value is None or value is Ellipsis:
        return ['none' if value is None else 'ellipsis']
    if isinstance(value, bool):
        return ['bool', value]
    if isinstance(value, int):
        return ['int', str(value)]
    if isinstance(value, float):
        return ['float', struct.pack('>d', value).hex()]
    if isinstance(value, complex):
        return ['complex', struct.pack('>dd', value.real, value.imag).hex()]
    if isinstance(value, bytes):
        return ['bytes', value.hex()]
    if isinstance(value, str):
        return ['str', value]
    if isinstance(value, tuple):
        return ['tuple', [constant(item) for item in value]]
    if isinstance(value, frozenset):
        items = [constant(item) for item in value]
        return ['frozenset', sorted(items, key=lambda item: json.dumps(item, sort_keys=True))]
    raise TypeError(type(value).__name__)


def code_hash(code):
    """🧾 Retain canonical live code components and every referenced nested code record."""
    cached = CODE_CACHE.get(id(code))
    if cached is not None:
        return cached[1]
    fields = ('co_argcount', 'co_posonlyargcount', 'co_kwonlyargcount', 'co_nlocals', 'co_stacksize',
              'co_flags', 'co_filename', 'co_name', 'co_qualname', 'co_firstlineno')
    record = {field: getattr(code, field) for field in fields}
    record.update({field: list(getattr(code, field)) for field in ('co_names', 'co_varnames', 'co_freevars', 'co_cellvars')})
    record.update({field: getattr(code, field).hex() for field in ('co_code', 'co_linetable', 'co_exceptiontable')})
    record['co_consts'] = [constant(value) for value in code.co_consts]
    digest = hashlib.sha256(json.dumps(record, sort_keys=True, separators=(',', ':')).encode()).hexdigest()
    CODE_CACHE[id(code)] = (code, digest)
    CODE_RECORDS[digest] = record
    return digest


def fresh_hashes(function, blobs):
    """🧬 Compare loaded code with fresh compilation of the exact current file bytes."""
    import types
    code = function.__code__
    file = relative_file(code.co_filename)
    blob = blobs.get(file)
    if blob is None:
        return None
    digest = hashlib.sha256(blob).hexdigest()
    key = (code.co_filename, digest)
    if key not in FRESH_CACHE:
        index = {}
        pending = [compile(blob.decode('utf-8'), code.co_filename, 'exec', dont_inherit=True, optimize=sys.flags.optimize)]
        while pending:
            current = pending.pop()
            index.setdefault(current.co_qualname, []).append(code_hash(current))
            pending.extend(value for value in current.co_consts if isinstance(value, types.CodeType))
        FRESH_CACHE[key] = index
    return FRESH_CACHE[key].get(code.co_qualname, [])


def function_record(function, blobs):
    """🔍 Describe actual callable code, current-source correspondence and bound defaults."""
    import inspect
    function = function.__func__ if inspect.ismethod(function) else function
    if not inspect.isfunction(function):
        return None
    digest = code_hash(function.__code__)
    fresh = fresh_hashes(function, blobs)
    cells = []
    for cell in function.__closure__ or ():
        try:
            cells.append(repr(cell.cell_contents))
        except ValueError:
            cells.append('<empty>')
    bindings = repr((function.__defaults__, function.__kwdefaults__, cells))
    return {'scope': 'repository' if relative_file(function.__code__.co_filename) in blobs else 'generated_or_framework', 'module': function.__module__, 'qualname': function.__qualname__, 'file': function.__code__.co_filename,
            'code_sha256': digest, 'bindings_sha256': hashlib.sha256(bindings.encode()).hexdigest(),
            'fresh_code_equal': None if fresh is None else digest in fresh, 'fresh_code_candidates': fresh}


def loaded_functions(test):
    """📚 Observe only repository-owned functions in loaded modules and their declared classes."""
    import inspect
    import types
    modules = list(sys.modules.values())
    api = getattr(test, 'api', None)
    if isinstance(api, types.ModuleType):
        modules.append(api)
    found = {}
    seen = set()
    for module in modules:
        if module is None or id(module) in seen:
            continue
        seen.add(id(module))
        file = relative_file(getattr(module, '__file__', None))
        if not file or not file.endswith('.py'):
            continue
        for name, value in list(vars(module).items()):
            members = [(name, value)]
            if inspect.isclass(value) and relative_file(getattr(sys.modules.get(value.__module__), '__file__', None)):
                members.extend((name + '.' + key, member.__func__ if isinstance(member, (staticmethod, classmethod)) else member)
                               for key, member in vars(value).items())
            for label, function in members:
                if not inspect.isfunction(function):
                    function = getattr(function, '__wrapped__', None)
                if inspect.isfunction(function) and relative_file(function.__code__.co_filename):
                    found[module.__name__ + '.' + label] = function
    return found


def snapshot(test, phase):
    """🚥 Capture exact per-test source bytes, loaded code and actual class-owned model identity."""
    try:
        blobs, sources = {}, {}
        for original_file in before:
            file = Path(original_file).as_posix()
            if Path(file).suffix not in {'.py', '.json', '.mjs', '.cjs', '.md', '.toml', '.lock'}:
                continue
            try:
                blob = (ROOT / file).read_bytes()
                blobs[file] = blob
                sources[file] = hashlib.sha256(blob).hexdigest()
            except OSError as error:
                sources[file] = {'error': str(error)}
        selected = []
        function = getattr(test, '_testFunc', None) or getattr(test, getattr(test, '_testMethodName', ''), None)
        while function is not None:
            record = function_record(function, blobs)
            if record:
                selected.append(record)
            function = getattr(function, '__wrapped__', None)
        functions = loaded_functions(test)
        loaded = {label: function_record(function, blobs) for label, function in functions.items()}
        model = getattr(test, 'model', None)
        model_digest = hashlib.sha256(json.dumps(model, sort_keys=True, separators=(',', ':'), allow_nan=False).encode()).hexdigest() if isinstance(model, dict) else None
        file = '🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json'
        source_model = json.loads(blobs[file].decode('utf-8'))
        source_digest = hashlib.sha256(json.dumps(source_model, sort_keys=True, separators=(',', ':'), allow_nan=False).encode()).hexdigest()
        record = {'phase': phase, 'started_at': start, 'id': canonical(test), 'unittest_id': test.id(), 'time': time.time(), 'sources': sources,
                  'selected_callables': selected, 'loaded_callables': loaded, 'loaded_class_model_sha256': model_digest,
                  'source_model_canonical_sha256': source_digest, 'class_model_matches_current_source': None if model_digest is None else model_digest == source_digest}
        with (OUTPUT / ('per-test-' + MODE + '.jsonl')).open('a', encoding='utf-8') as stream:
            stream.write(json.dumps(record, ensure_ascii=True) + '\n')
    except Exception as error:
        import traceback
        detail = traceback.format_exc()
        OBSERVER_ERRORS.append({'id': canonical(test), 'phase': phase, 'error': detail})
        print('[DEBUG] Native provenance observer error: ' + detail, flush=True)


class Result(unittest.TextTestResult):
    """🚦 Publish bounded per-test progress and allow cooperative cancellation."""
    def stopTest(self, test):
        snapshot(test, "after")
        super().stopTest(test)

    def record_issue(self, test, error, kind):
        detail = self._exc_info_to_string(error, test)
        with (OUTPUT / ('issues-' + MODE + '.jsonl')).open('a', encoding='utf-8') as stream:
            stream.write(json.dumps({'started_at': start, 'id': canonical(test), 'unittest_id': test.id(), 'kind': kind, 'error': detail, 'time': time.time()}) + '\n')
        print('[DEBUG] Native ' + kind + ': ' + detail, flush=True)

    def addFailure(self, test, error):
        self.record_issue(test, error, 'failure')
        super().addFailure(test, error)

    def addError(self, test, error):
        self.record_issue(test, error, 'error')
        super().addError(test, error)

    def addSkip(self, test, reason):
        print("[DEBUG] Native skipped " + canonical(test) + ": " + str(reason), flush=True)
        super().addSkip(test, reason)

    def addSuccess(self, test):
        self.passed = getattr(self, "passed", []) + [canonical(test)]
        super().addSuccess(test)

    def addSubTest(self, test, subtest, err):
        if err is not None:
            self.record_issue(subtest, err, 'subtest')
        super().addSubTest(test, subtest, err)

    def startTest(self, test):
        if (TICKET / 'native-r92-cancel').exists():
            raise KeyboardInterrupt('Native validation cancelled')
        (OUTPUT / ('progress-' + MODE + '.json')).write_text(json.dumps({'state': 'running', 'test': canonical(test), 'time': time.time()}), encoding='utf-8')
        print('[DEBUG] Native BIM ' + canonical(test), flush=True)
        snapshot(test, "before")
        self.observed = getattr(self, "observed", []) + [canonical(test)]
        super().startTest(test)

before = hashes()
(OUTPUT / ('inputs-before-' + MODE + '.json')).write_text(json.dumps(before, indent=2), encoding='utf-8')
start = time.time()
module = '🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️'
selected = [module + '.BimBrowserMetadataContract.test_source_discovery_skips_browser_runtime', module + '.BimClipContract.test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences', module + '.BimClipContract.test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene'] if MODE == 'gates' else ['🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🎭️presentation.🐍️.BimPresentationContract.test_native_lifecycle']
if MODE == 'chapters':
    selected = ['🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🧩️chapter-source.🐍️.BimChapterSourceContract']
if MODE == 'camera':
    selected = ['🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🎥️camera.🧪️tests.🐍️']
elif MODE == 'observer':
    selected = ['🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🧩️chapter-source.🐍️.BimChapterSourceContract.test_information_grid_highlights_follow_coordinates_instead_of_support_names']
elif MODE in ('all', 'diagnostic', 'current', 'discover'):
    selected = [module]
suite = unittest.TestLoader().loadTestsFromNames(selected)
def flatten(suite):
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            yield from flatten(item)
        else:
            yield item
manifest = [{'id': canonical(test), 'unittest_id': test.id()} for test in flatten(suite)]
(OUTPUT / ('discovery-' + MODE + '.json')).write_text(json.dumps(manifest, indent=2), encoding='utf-8')
assert len({row['id'] for row in manifest})==len(manifest),'Native discovery identities must be unique'
if MODE=='discover':
    after=hashes()
    report={'success':False,'discovery_only':True,'discovery_success':before==after,'discovered_count':len(manifest),'started_at':start,'inputs_before':before,'inputs_after':after,'manifest':manifest}
    (OUTPUT/'result-discover.json').write_text(json.dumps(report,indent=2),encoding='utf8')
    print('[DEBUG] Native discovery only: '+str(len(manifest))+' unique IDs',flush=True)
    raise SystemExit(0 if report['discovery_success'] else 1)
stopped = threading.Event()
def observe_cancel():
    """🛑 Cancel even a long native case without publishing incomplete geometry as a successful cohort."""
    while not stopped.wait(.5):
        if (TICKET / "native-r92-cancel").exists():
            _thread.interrupt_main()
            return
threading.Thread(target=observe_cancel, daemon=True).start()
try:
    result = unittest.TextTestRunner(verbosity=2, resultclass=Result).run(suite)
except BaseException as error:
    (OUTPUT / ("result-" + MODE + ".json")).write_text(json.dumps({"success":False,"interrupted":True,"started_at":start,"mode":MODE,"error":repr(error),"inputs_before":before}), encoding="utf8")
    raise
finally:
    stopped.set()
after = hashes()
import importlib.metadata
versions = {name: importlib.metadata.version(name) for name in ('manim', 'numpy', 'scipy', 'shapely', 'Pillow', 'pycairo', 'manimpango', 'imageio-ffmpeg', 'jsonschema')}
report = {'started_at': start, 'python': sys.version, 'executable': sys.executable, 'packages': versions, 'mode': MODE, 'diagnostic': MODE != 'all', 'observer_errors': OBSERVER_ERRORS, 'selected': selected, 'observed': getattr(result, 'observed', []), 'passed': getattr(result, 'passed', []), 'run': result.testsRun, 'skipped': [(canonical(t), reason) for t,reason in result.skipped], 'errors': [(canonical(t), e) for t,e in result.errors], 'failures': [(canonical(t), e) for t,e in result.failures], 'seconds': time.time()-start, 'inputs_before': before, 'inputs_after': after, 'stable': before == after, 'tests_successful': result.wasSuccessful(), 'success': result.wasSuccessful() and before == after and not OBSERVER_ERRORS and not result.skipped}
(OUTPUT / ('loaded-code-' + MODE + '.json')).write_text(json.dumps(CODE_RECORDS, indent=2), encoding='utf-8')
(OUTPUT / ('result-' + MODE + '.json')).write_text(json.dumps(report, indent=2), encoding='utf-8')
raise SystemExit(0 if report['success'] else 1)
