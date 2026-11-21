from pathlib import Path
import csv
import sqlite3
import subprocess
import sys


def main():
    ticket = Path(__file__).resolve().parent
    root = ticket.parents[6]
    artifact = root / '✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805'
    inferences = artifact / '🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences'
    fixture = inferences / '🧪️tests/🔬️unit/🧫️fixtures/part-100-components.tsv'
    with fixture.open(newline='', encoding='utf-8') as stream:
        cases = list(csv.DictReader(stream, delimiter='\t'))
    production = (inferences / '🧱️sheet-checks/🦀️.rs').read_text(encoding='utf-8')
    rust_predicate = production.split('fn check_system_component_references', 1)[1].split('let valid = ', 1)[1].split('    let check = ', 1)[0].strip()
    generated = ticket / '🗑️generated'
    source = generated / 'vdi3805-part100-scope.rs'
    binary = generated / ('vdi3805-part100-scope.exe' if sys.platform == 'win32' else 'vdi3805-part100-scope')
    source.write_text('''
struct Link { component_id: String }
struct Product { components: Vec<Link> }
fn main() {
    for record in std::env::args().skip(1) {
        let article = String::from("package");
        let product = Product { components: record.split(',').filter(|id| !id.is_empty()).map(|id| Link { component_id: id.into() }).collect() };
        let valid = ''' + rust_predicate + '''
        println!("{}", if valid { "pass" } else { "fail" });
    }
}
''', encoding='utf-8')
    subprocess.run(['rustc', '--edition=2021', str(source), '-o', str(binary)], check=True)
    rust = subprocess.check_output([str(binary), *(case['component_ids'] for case in cases)], text=True).splitlines()
    with sqlite3.connect(':memory:') as database:
        database.execute('CREATE TABLE links (component TEXT NOT NULL)')
        for case, actual in zip(cases, rust):
            database.execute('DELETE FROM links')
            database.executemany('INSERT INTO links VALUES (?)', [(part,) for part in case['component_ids'].split(',') if part])
            (oracle,) = database.execute("SELECT CASE WHEN COUNT(*) = 0 OR SUM(TRIM(component) IN ('', 'package')) > 0 THEN 'fail' ELSE 'pass' END FROM links").fetchone()
            assert actual == oracle == case['status'], (case, actual, oracle)
            print(f"[DEBUG] {case['component_ids']!r}: Rust and SQLite {actual}")
    assert len(rust) == len(cases) > 0


if __name__ == '__main__':
    main()
