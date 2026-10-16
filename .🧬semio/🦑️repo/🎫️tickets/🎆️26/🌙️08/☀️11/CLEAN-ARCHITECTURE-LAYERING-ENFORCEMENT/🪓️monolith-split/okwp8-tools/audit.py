"""🔍 Binding audit of the staged crates: capitalized/const identifiers that no scope of the crate binds."""
import sys, os, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + '/../okwp0-tools')
import wp8lib as L, usetree, unitgraph
from okrs import tokenize
STD = set('''Self Option Some None Result Ok Err Vec String Box Arc Rc Cell RefCell Default Clone Copy Debug Display PartialEq Eq PartialOrd Ord Hash Send Sync Sized Drop Fn FnMut FnOnce Iterator IntoIterator Into From TryFrom TryInto AsRef AsMut ToString ToOwned Extend
Future Send Unpin Pin Poll Duration Instant Cow Path PathBuf Ordering Infallible ManuallyDrop PhantomData Weak Mutex RwLock Condvar OnceLock LazyLock AtomicBool AtomicUsize AtomicU64 AtomicU32 AtomicI64 AtomicIsize Error Write Read BufRead Seek HashMap HashSet BTreeMap BTreeSet VecDeque BinaryHeap
Bytes Layout NonNull Wrapping Reverse Range RangeInclusive TypeId Any Formatter Arguments Utf8Error ParseIntError File OpenOptions Command Stdio'''.split())
KEYWORD_LIKE = {'True', 'False'}
def items_of(text):
    return unitgraph.parse_text(text) if hasattr(unitgraph, 'parse_text') else None
def defined_names(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    names = set()
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] in ('struct', 'enum', 'trait', 'type', 'const', 'static', 'fn', 'mod', 'union', 'macro_rules') and i + 1 < len(toks):
            j = i + 1
            if t[1] == 'macro_rules': j = i + 2
            if toks[j][0] == 'id': names.add(toks[j][1])
    # generics and enum variants, fields: approximate by every identifier appearing after `<` or `,` inside angle brackets of impl/fn/struct headers
    for m in re.finditer(r'<\s*([A-Z][A-Za-z0-9_]*)\b', text): names.add(m.group(1))
    for m in re.finditer(r',\s*([A-Z][A-Za-z0-9_]*)\s*(?:[:,>=])', text): names.add(m.group(1))
    for m in re.finditer(r"'static|\bconst\s+([A-Z][A-Z0-9_]*)\s*:", text): 
        if m.group(1): names.add(m.group(1))
    # enum variants: identifiers at line start inside enum bodies
    for m in re.finditer(r'enum\s+\w+[^{]*\{([^}]*)\}', text, re.S):
        for v in re.finditer(r'(?:^|[,{])\s*([A-Z][A-Za-z0-9_]*)', m.group(1)): names.add(v.group(1))
    return names
def use_leaves(text):
    names = set(); 
    for m in re.finditer(r'(?:^|[;{}\s])(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);', text):
        try: leaves = usetree.parse_use('use ' + m.group(1) + ';')
        except Exception: continue
        for segs, alias, glob in leaves:
            if glob: continue
            nm = alias or (segs[-2] if segs[-1] == 'self' and len(segs) > 1 else segs[-1])
            names.add(nm)
    return names
def pub_use_globs(text):
    return [m.group(1) for m in re.finditer(r'use\s+([A-Za-z0-9_:]+)::\*\s*;', text)]
def free_caps(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = collections.Counter()
    for i, t in enumerate(toks):
        if t[0] != 'id': continue
        s = t[1]
        if not (s[0].isupper()): continue
        prev = toks[i - 1][1] if i else ''
        prev2 = toks[i - 2][1] if i > 1 else ''
        nxt = toks[i + 1][1] if i + 1 < len(toks) else ''
        if prev == '.': continue
        if prev == ':' and prev2 == ':': continue
        if nxt == ':' and (i + 2 < len(toks) and toks[i + 2][1] != ':'): continue      # struct field `Name: type` (rare) / label
        if s in STD: continue
        out[s] += 1
    return out
if __name__ == '__main__':
    pass

def glob_sources(text):
    out = []
    for m in re.finditer(r'(?:^|[;{}\s])use\s+([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)::\*\s*;', text):
        out.append(m.group(1).split('::'))
    return out
def audit_texts(files, extra_bound, exists):
    """files: {path: text}; returns {path: {name: count}} of capitalized identifiers bound by nothing in the crate."""
    bound = set(extra_bound)
    globs = []
    for p, t in files.items():
        bound |= defined_names(t) | use_leaves(t)
        globs += glob_sources(t)
    res = {}
    for p, t in files.items():
        fc = free_caps(t)
        miss = {}
        for k, v in fc.items():
            if k in bound: continue
            if any(exists(g + [k]) is True for g in globs if g and g[0] not in ('super', 'self', 'crate')): continue
            miss[k] = v
        if miss: res[p] = miss
    return res

KW = set('as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try union'.split())
PRIM = set('u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 bool char str'.split())
STD_FN = set('drop vec format println eprintln print eprint assert assert_eq assert_ne debug_assert debug_assert_eq debug_assert_ne panic unreachable unimplemented todo matches write writeln concat stringify include_str include_bytes include env cfg line file column module_path dbg compile_error thread_local macro_rules std core alloc serde_json serde tokio futures vec_deque'.split())
def local_bindings(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    names = set()
    for i, t in enumerate(toks):
        if t[0] != 'id': continue
        s = t[1]
        nxt = toks[i + 1][1] if i + 1 < len(toks) else ''
        prev = toks[i - 1][1] if i else ''
        if nxt == ':' and (i + 2 >= len(toks) or toks[i + 2][1] != ':'): names.add(s)       # param / field / label
        if prev in ('let', 'mut', 'ref', '|', ',', '(', '{', '&') and nxt in ('=', ',', ')', '|', ':', ';', '@') : names.add(s)
        if prev == 'for' or prev == 'in': names.add(s)
    # closure params |a, b|
    for m in re.finditer(r'\|([^|]*)\|', text):
        for n in re.findall(r'[A-Za-z_][A-Za-z0-9_]*', m.group(1)): names.add(n)
    # match arms/pattern bindings are common: Some(x) etc.
    for m in re.finditer(r'(?:Some|Ok|Err)\(\s*(?:mut\s+)?([a-z_][A-Za-z0-9_]*)', text): names.add(m.group(1))
    return names
def free_lower(text):
    toks = [t for t in tokenize(text) if t[0] not in ('ws', 'comment')]
    out = collections.Counter()
    for i, t in enumerate(toks):
        if t[0] != 'id': continue
        s = t[1]
        if s[0].isupper() or s in KW or s in PRIM or s in STD_FN: continue
        prev = toks[i - 1][1] if i else ''
        prev2 = toks[i - 2][1] if i > 1 else ''
        nxt = toks[i + 1][1] if i + 1 < len(toks) else ''
        nxt2 = toks[i + 2][1] if i + 2 < len(toks) else ''
        if prev == '.' or (prev == ':' and prev2 == ':'): continue
        if prev in ('fn', 'mod', 'let', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'use', 'macro_rules', 'as', "'"): continue
        callish = nxt == '(' or nxt == '!' or (nxt == ':' and nxt2 == ':')
        if not callish: continue
        out[s] += 1
    return out
def audit_lower(files, extra_bound, exists):
    bound = set(extra_bound); globs = []
    for p, t in files.items():
        bound |= defined_names(t) | use_leaves(t) | local_bindings(t) | set(re.findall(r'\bfn\s+([a-z_][A-Za-z0-9_]*)', t)) | set(re.findall(r'\bmod\s+([a-z_][A-Za-z0-9_]*)', t)) | set(re.findall(r'macro_rules!\s*([A-Za-z_][A-Za-z0-9_]*)', t))
        globs += glob_sources(t)
    res = {}
    for p, t in files.items():
        miss = {}
        for k, v in free_lower(t).items():
            if k in bound: continue
            if any(exists(g + [k]) is True for g in globs if g and g[0] not in ('super', 'self', 'crate')): continue
            miss[k] = v
        if miss: res[p] = miss
    return res

import unitgraph
def impl_rows(text):
    src_items = None
    from okrs import items as _items
    rows = []
    for it in _items(text):
        if it['kind'] == 'impl':
            tr, st = unitgraph.impl_parts(it['text'])
            rows.append((tr, st, it['text'][:120].replace('\n', ' ')))
    return rows
def type_defs(text):
    out = set()
    for m in re.finditer(r'\b(?:struct|enum|trait|union|type)\s+([A-Z][A-Za-z0-9_]*)', text): out.add(m.group(1))
    return out
def orphan_audit(files):
    local = set()
    for t in files.values(): local |= type_defs(t)
    bad = []
    for p, t in files.items():
        for tr, st, head in impl_rows(t):
            if tr is None: continue            # inherent impl: needs local type
            if tr in local or st in local: continue
            bad.append((p, tr, st, head))
    inh = []
    for p, t in files.items():
        for tr, st, head in impl_rows(t):
            if tr is None and st not in local: inh.append((p, st, head))
    return bad, inh
