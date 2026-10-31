"""🧭 Fingerprint the actual local dependency closure of a native scene."""
import ast
import hashlib
import json
from pathlib import Path

def local_imports(root,path):
    """🔗 Resolve literal repository imports without importing their modules."""
    if path.suffix!='.py':
        return set()
    found=set()
    for node in ast.walk(ast.parse(path.read_text(encoding='utf-8'))):
        names=[]
        if isinstance(node,ast.Import):
            names=[item.name for item in node.names]
        elif isinstance(node,ast.ImportFrom) and node.module and not node.level:
            names=[node.module]
        elif isinstance(node,ast.ImportFrom) and node.level:
            base=path.parent
            for _ in range(node.level-1):
                base=base.parent
            target=base.joinpath(*(node.module or '').split('.'))
            for candidate in (target.with_suffix('.py'),target/'__init__.py'):
                if candidate.is_file():
                    found.add(candidate.resolve())
        elif isinstance(node,ast.Call) and node.args and isinstance(node.args[0],ast.Constant) and isinstance(node.args[0].value,str) and getattr(node.func,'attr','')=='import_module':
            names=[node.args[0].value]
        for name in names:
            base=root.joinpath(*name.split('.'))
            for candidate in (base.with_suffix('.py'),base/'__init__.py'):
                if candidate.is_file():
                    found.add(candidate.resolve())
    return found

def dependency_paths(root,selected,manifest,harness):
    """🕸️ Include selected sources, declared visual inputs, harnesses and their local imports."""
    pending=[Path(p).resolve() for p in (*selected,*manifest,*harness)]
    found=set()
    while pending:
        path=pending.pop()
        if path in found:
            continue
        if not path.is_file():
            raise FileNotFoundError(path)
        found.add(path)
        pending.extend(local_imports(root,path)-found)
    return tuple(sorted(found,key=str))

def fingerprints(root,paths):
    """🔐 Ignore formatting while preserving Python, JSON and binary input semantics."""
    result={}
    for path in paths:
        if path.suffix=='.py':
            value=ast.dump(ast.parse(path.read_text(encoding='utf-8')),include_attributes=False).encode()
        elif path.suffix=='.json':
            value=json.dumps(json.loads(path.read_text(encoding='utf-8')),sort_keys=True,ensure_ascii=False,separators=(',',':')).encode()
        else:
            value=path.read_bytes()
        key=str(path.relative_to(root)) if path.is_relative_to(root) else '@external:'+str(path)
        result[key]=hashlib.sha256(value).hexdigest()
    return result

def unchanged(start,end):
    """⚖️ Require explicit equality of both captured dependency maps."""
    return bool(start) and start==end

def render_configurations(path):
    """🎞️ Record explicit native renderer settings declared by the fingerprinted harness."""
    configurations=[]
    for node in ast.walk(ast.parse(path.read_text(encoding='utf-8'))):
        if not isinstance(node,ast.Dict):
            continue
        values={}
        for key,value in zip(node.keys,node.values):
            if not isinstance(key,ast.Constant) or not isinstance(key.value,str):
                continue
            try:
                values[key.value]=ast.literal_eval(value)
            except (ValueError,TypeError):
                values[key.value]={'expression':ast.unparse(value)}
        if 'pixel_width' in values and 'pixel_height' in values and values not in configurations:
            configurations.append(values)
    return configurations
