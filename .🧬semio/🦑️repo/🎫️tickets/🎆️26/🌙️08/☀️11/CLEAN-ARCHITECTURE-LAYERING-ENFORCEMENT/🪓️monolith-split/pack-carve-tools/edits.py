"""✏️ Anchored text edits of live shared files (scripts, tests, fixtures); every edit asserts its anchor count."""
import re, json
from lib import *
from crates import *

ALL_PACKAGES = [cname(k) for k, *_ in CRATES] + ["semio-framework-pack"]

def q(names, quote='"'):
    return "[" + ",".join(quote + n + quote for n in names) + "]"

def retarget(text, literal, packages, quote='"'):
    i = text.index(literal)
    assert text.count(literal) == 1, literal
    j = text.rindex("packages", 0, i)
    m = re.compile(r"packages\s*:\s*\[[^\]]*\]").match(text, j)
    assert m, (literal, text[j:j + 80])
    return text[:m.start()] + "packages:" + q(packages, quote) + text[m.end():]

def facade_script(text):
    # package list constant
    anchor = "class TestScript extends BundleScript {"
    assert text.count(anchor) == 1
    const = ('/** 🎒️ The pack crates bottom up: container core, retained cursors, record codec stack, transports, then the façade every consumer links. */\n'
             'const PACK_PACKAGES = ' + q(ALL_PACKAGES) + ';\n\n')
    text = text.replace(anchor, const + anchor)
    a = 'packages: ["semio-framework-pack"], cwd: this.root, extraArgs: rest }'
    assert text.count(a) == 1
    text = text.replace(a, 'packages: PACK_PACKAGES, cwd: this.root, extraArgs: rest }')
    text = retarget(text, '"pack_absolute_unsigned_varint_complete_boundaries"', [cname("format")])
    old = 'await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-pack"],cwd:this.root,extraArgs:["--lib","record_borrowed_preflight_","--no-fail-fast"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation);'
    assert text.count(old) == 1
    new = ('await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["' + cname("preflight") + '"],cwd:this.root,extraArgs:["--lib","record_borrowed_preflight_","--no-fail-fast"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation);\n'
           '    await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-pack"],cwd:this.root,extraArgs:["--test","pack_record_laws","record_borrowed_preflight_","--no-fail-fast"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation);')
    text = text.replace(old, new)
    old = 'extraArgs:["--lib","schema_hash_controlled_full_allocator_requests_and_same_caller_are_admitted","--no-fail-fast"]'
    assert text.count(old) == 1
    text = text.replace(old, 'extraArgs:["--test","pack_record_schema_laws","schema_hash_controlled_full_allocator_requests_and_same_caller_are_admitted","--no-fail-fast"]')
    text = retarget(text, "'pack_original_value_stack_storage_'", [cname("rretained")], quote="'")
    text = retarget(text, '"retained_intrinsic_"', [cname("intrinsic")])
    text = retarget(text, '"intrinsic_document_schema_refusal_"', [cname("intrinsic")])
    text = retarget(text, '"original_record_receiving_"', [cname("rretained")])
    return text

def record_script(text):
    old = 'await runCargoTestsV1({manifestPath,packages:["semio-framework-pack"],cwd:resolve(this.root,"../../../📦️packages/🦀️rust"),extraArgs:["--lib","record::",...rest]},readCargoTestPolicyV1(process.env),this.repoRoot, this.invocation);'
    assert text.count(old) == 1
    rec = [cname(k) for k in ("schema", "rretained", "encode", "preflight", "record")]
    new = ('const policy=readCargoTestPolicyV1(process.env);\n'
           '    await runCargoTestsV1({manifestPath,packages:' + q(rec) + ',cwd:resolve(this.root,"../../../📦️packages/🦀️rust"),extraArgs:["--lib",...rest]},policy,this.repoRoot, this.invocation);\n'
           '    await runCargoTestsV1({manifestPath,packages:["semio-framework-pack"],cwd:resolve(this.root,"../../../📦️packages/🦀️rust"),extraArgs:["--test","pack_record_laws","--test","pack_record_schema_laws",...rest]},policy,this.repoRoot, this.invocation);')
    return text.replace(old, new)

def root_script(text):
    for a, b in (('self.begin_identity_segment(crate::KIND_CHUNK"', 'self.begin_identity_segment(protocol::codec::KIND_CHUNK"'),
                 ('"encode_segment(crate::KIND_CHUNK_TABLE"', '"encode_segment(protocol::codec::KIND_CHUNK_TABLE"'),
                 ('"encode_segment(crate::KIND_MANIFEST"', '"encode_segment(protocol::codec::KIND_MANIFEST"')):
        assert text.count(a) == 1, a
        text = text.replace(a, b)
    return text

def db_test(text):
    a = 'self.begin_identity_segment(crate::KIND_CHUNK"'
    assert text.count(a) == 1
    return text.replace(a, 'self.begin_identity_segment(protocol::codec::KIND_CHUNK"')

def cause_fixture(text):
    """Rows whose predicate lives in the retained source cursor now point at the retained crate's source."""
    moved = ["RetainedPackSourceCursor::grant observes cancelled", "next_allocation_bytes closed/sealed branch after distinct canceled precedence", "requested_allocation_bytes receives Ok(None) while allocation_need is Some"]
    old = '"path": "🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs"'
    new = '"path": "🧰️framework/🔨️modules/🎒️pack/📐️format/🎮️retained/🦀️.rs"'
    for p in moved:
        i = text.index('"predicate": "' + p + '"')
        j = text.rindex(old, 0, i)
        assert i - j < 400, (p, i - j)
        text = text[:j] + new + text[j + len(old):]
    json.loads(text)
    return text

def source_fixture(text):
    """The nominal byte owner pack-source is now consumed by the format, retained, catalog and io crates instead of the façade."""
    old = '    "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml",\n'
    assert text.count(old) == 1
    new = "".join('    "' + pkgdir(k) + '/Cargo.toml",\n' for k in ("format", "retained", "catalog", "io"))
    out = text.replace(old, new)
    json.loads(out)
    return out
