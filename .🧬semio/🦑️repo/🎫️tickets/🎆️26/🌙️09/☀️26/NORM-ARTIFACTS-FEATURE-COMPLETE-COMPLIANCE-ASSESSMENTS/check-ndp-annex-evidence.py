from pathlib import Path
import json
import sqlite3
import subprocess
import sys


def main():
    ticket = Path(__file__).resolve().parent
    registry = ticket.parents[6] / '✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp'
    g = ticket / '🗑️generated'
    model = g / 'libsemio_s_plugin_norm_ndp_model.rlib'
    table = g / 'libsemio_s_plugin_norm_ndp_en1992.rlib'
    source = g / 'ndp-annex-evidence.rs'
    binary = g / ('ndp-annex-evidence.exe' if sys.platform == 'win32' else 'ndp-annex-evidence')
    rustc = ['rustc', '--edition=2021']
    subprocess.run([*rustc, '--crate-type=rlib', '--crate-name=semio_s_plugin_norm_ndp_model', str(registry/'🧱️model/🦀️.rs'), '-o', str(model)], check=True)
    subprocess.run([*rustc, '--crate-type=rlib', '--crate-name=semio_s_plugin_norm_ndp_en1992', '--extern', f'semio_s_plugin_norm_ndp_model={model}', str(registry/'🗃️table/en1992/🦀️.rs'), '-o', str(table)], check=True)
    source.write_text('''
use semio_s_plugin_norm_ndp_en1992::ROWS;
use semio_s_plugin_norm_ndp_model::{d, NdpStatus, NdpValue};
fn main() {
    let missing_de = d("demo", "EN", "1", "x", NdpValue::Number(1.0), NdpValue::Unset, "", "EN").ver();
    assert_eq!((missing_de.status_en, missing_de.status_de, missing_de.status), (NdpStatus::Verified, NdpStatus::Unverified, NdpStatus::Unverified));
    let missing_en = d("demo", "EN", "1", "x", NdpValue::Unset, NdpValue::Number(1.0), "", "EN").ver();
    assert_eq!((missing_en.status_en, missing_en.status_de, missing_en.status), (NdpStatus::Unverified, NdpStatus::Verified, NdpStatus::Unverified));
    for row in ROWS {
        println!("{}\\t{}\\t{}\\t{}", row.id, u8::from(row.status_en == NdpStatus::Verified), u8::from(row.status_de == NdpStatus::Verified), u8::from(row.status == NdpStatus::Verified));
    }
}
''', encoding='utf-8')
    subprocess.run([*rustc, '-L', f'dependency={g}', '--extern', f'semio_s_plugin_norm_ndp_model={model}', '--extern', f'semio_s_plugin_norm_ndp_en1992={table}', str(source), '-o', str(binary)], check=True)
    rust = {}
    for line in subprocess.check_output([str(binary)], text=True).splitlines():
        name, *values = line.split('\t')
        rust[name] = tuple(map(int, values))
    data = (registry/'🔣️.json').read_text(encoding='utf-8')
    with sqlite3.connect(':memory:') as db:
        expected = {name: (en, de, joint) for name, en, de, joint in db.execute('''
          WITH registry AS (
            SELECT json_extract(value,'$.id') AS id,
                   json_extract(value,'$.recommended') AS en_value, json_extract(value,'$.naDe') AS de_value,
                   json_extract(value,'$.status') AS joint_status,
                   json_extract(value,'$.annexStatus.en.status') AS en_status,
                   json_extract(value,'$.annexStatus.naDe.status') AS de_status
            FROM json_each(?, '$.entries')
          ), selected AS (
            SELECT id, (en_value IS NOT NULL AND coalesce(en_status, joint_status)='verified') AS en,
                       (de_value IS NOT NULL AND coalesce(de_status, joint_status)='verified') AS de
            FROM registry WHERE id LIKE 'ndp.en1992.%'
          )
          SELECT id,en,de,en AND de FROM selected
        ''', (data,))}
    assert rust == expected and rust, {k:(v,expected.get(k)) for k,v in rust.items() if expected.get(k)!=v}
    print(f'[DEBUG] Rust EN 1992 table and SQLite agree on all {len(rust)} rows. Both missing-value constructor cases passed.')


if __name__ == '__main__':
    main()
