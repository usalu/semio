import json, pathlib, subprocess, sys

HERE = pathlib.Path(__file__).parent
sys.path.insert(0, str(HERE))
import fixtures

TOP_TESTS = fixtures.ROOT / '🧪️tests' / '🔬️unit' / '🦀️.rs'
BEGIN = '// [DEBUG] regen begin'
END = '// [DEBUG] regen end'
TYPES = {
    's.stdio.semio.animation': 'animation::schema::snapshot::SemioAnimationSnapshot',
    's.stdio.semio.document': 'document::schema::snapshot::SemioDocumentSnapshot',
    's.stdio.semio.graph': 'graph::schema::snapshot::SemioGraphSnapshot',
    's.stdio.semio.image': 'image::schema::snapshot::SemioImageSnapshot',
    's.stdio.semio.presentation': 'presentation::schema::snapshot::SemioPresentationSnapshot',
    's.stdio.semio.table': 'table_model::schema::snapshot::SemioTableSnapshot',
    's.stdio.semio.text': 'text_model::schema::snapshot::SemioTextSnapshot',
    'stdio.semio.audio': 'audio_model::schema::snapshot::SemioAudioSnapshot',
    'stdio.semio.brep': 'brep::schema::snapshot::SemioBrepSnapshot',
    'stdio.semio.cad': 'cad::schema::snapshot::SemioCadSnapshot',
    'stdio.semio.drawing': 'drawing::schema::snapshot::SemioDrawingSnapshot',
    'stdio.semio.kit': 'kit::schema::snapshot::SemioKitSnapshot',
    'stdio.semio.mesh': 'mesh::schema::snapshot::SemioMeshSnapshot',
    'stdio.semio.model': 'model::schema::snapshot::SemioModelSnapshot',
    'stdio.semio.object': 'object::schema::snapshot::SemioObjectSnapshot',
    'stdio.semio.value': 'value_model::schema::snapshot::SemioValueSnapshot',
    'stdio.semio.video': 'video_model::schema::snapshot::SemioVideoSnapshot',
}


def rows():
    plan = fixtures.plan()
    return [r for r in plan if r['env'] in TYPES and r['dsl']]


def render(plan_rows):
    arms = '\n'.join('        "%s" => enc::<semio_s_artifact_stdio_semio_%s::%s>(path),' % (env, ty.split('::', 1)[0], ty.split('::', 1)[1]) for env, ty in TYPES.items())
    table = ',\n'.join('        ("%s", "%s")' % (r['env'], r['dsl']) for r in plan_rows)
    return f'''{BEGIN}
#[test]
#[ignore]
fn debug_regen_pack_fixtures() {{
    fn enc<S: ::pack::snapshot::ArtifactDsl + ::pack::snapshot::ArtifactPack>(path: &str) -> String {{
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{{path}}: {{e}}"));
        let snapshot = S::parse_dsl(&text).unwrap_or_else(|e| panic!("{{path}}: {{e}}"));
        S::encode_pack(&snapshot).iter().map(|b| format!("{{b:02x}}")).collect()
    }}
    let rows: &[(&str, &str)] = &[
{table},
    ];
    for (env, path) in rows {{
        let hex = match *env {{
{arms}
        _ => panic!("{{env}}"),
        }};
        println!("[DEBUG] REGEN|{{path}}|{{hex}}");
    }}
}}
{END}
'''


if __name__ == '__main__':
    mode = sys.argv[1]
    if mode == 'patch':
        plan_rows = rows()
        code = render(plan_rows)
        text = TOP_TESTS.read_text(encoding='utf-8')
        assert BEGIN not in text
        TOP_TESTS.write_text(text + '\n' + code, encoding='utf-8')
        json.dump(plan_rows, open(sys.argv[2], 'w'), ensure_ascii=False)
    elif mode == 'unpatch':
        text = TOP_TESTS.read_text(encoding='utf-8')
        i = text.index(BEGIN)
        TOP_TESTS.write_text(text[:i].rstrip('\n') + '\n', encoding='utf-8')
    elif mode == 'apply':
        plan_rows = json.load(open(sys.argv[2]))
        by_dsl = {r['dsl']: r for r in plan_rows}
        count = 0
        for line in pathlib.Path(sys.argv[3]).read_text(encoding='utf-8', errors='replace').split('\n'):
            if line.startswith('[DEBUG] REGEN|'):
                _, dsl, hexed = line.split('|', 2)
                for target in by_dsl[dsl]['targets']:
                    pathlib.Path(target).write_bytes(bytes.fromhex(hexed.strip()))
                    count += 1
        print('wrote', count)
