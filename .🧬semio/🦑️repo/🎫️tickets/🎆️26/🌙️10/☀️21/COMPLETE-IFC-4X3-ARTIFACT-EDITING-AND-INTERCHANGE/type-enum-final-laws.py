from pathlib import Path
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details')
f=p/'type_enum_input.test.ts';s=f.read_text(encoding='utf8');a="const next=form.pageBindings.find(binding=>binding.page.choice?.offset==='31')!;current=advance";b="const next=form.pageBindings.find(binding=>binding.page.choice?.offset==='31')!;expect(next.page.choice?.path).toEqual(['entities','0','args',String(row.required[0]!.index)]);current=advance";assert a in s;s=s.replace(a,b);f.write_text(s,encoding='utf8')
f=p/'🧪️tests/type-enum-input.feature';s=f.read_text(encoding='utf8');s+='''\n  Scenario: An earlier required choice preserves an invalid UTF8 sibling draft
    Given a later logical input contains the exact neutral multibyte draft text
    When the caller chooses U for an earlier missing logical input
    Then the resident fixed binding retains sorted target indices and every sibling UTF8 byte
    And readiness stays false without promoting the unknown raw word to a declared option
  Scenario: Unchanged type completion emits no name scaffolding
    Given the original declaration and direct-layout proof admit an unchanged type repair
    When required caller choices complete its strict candidate tuple
    Then the central batch contains only actual changed SetEntityArg atoms
    And the inverse restores original unset slots without any identity SetEntityName atom
    And page coordinates identify candidate target attributes independently of raw input row ordinals
''';f.write_text(s,encoding='utf8')
