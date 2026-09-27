"""N1 one-off codemod: rewrites the interpolated norm mutation labels (en1992, en1993, iso16757, vdi3805) whose English
or German text carried raw field names, kebab ids or untranslated English nouns."""
import glob, re, sys
ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
write = "--write" in sys.argv
EN1992 = {
    '"Change {} of {}", "width", self.member_id), &format!("{} von {} ändern", "width", self.member_id)': '"Change width of member {}", self.member_id), &format!("Breite von Bauteil {} ändern", self.member_id)',
    '"Change {} of {}", "height", self.member_id), &format!("{} von {} ändern", "height", self.member_id)': '"Change height of member {}", self.member_id), &format!("Höhe von Bauteil {} ändern", self.member_id)',
    '"Change {} of {}", "span", self.member_id), &format!("{} von {} ändern", "span", self.member_id)': '"Change span of member {}", self.member_id), &format!("Stützweite von Bauteil {} ändern", self.member_id)',
    '"Change {} of {}", "cover", self.member_id), &format!("{} von {} ändern", "cover", self.member_id)': '"Change nominal concrete cover of member {}", self.member_id), &format!("Nennmaß der Betondeckung von Bauteil {} ändern", self.member_id)',
    '"Change {} of {}", "effective_depth", self.member_id), &format!("{} von {} ändern", "effective_depth", self.member_id)': '"Change effective depth of member {}", self.member_id), &format!("Statische Nutzhöhe von Bauteil {} ändern", self.member_id)',
    '"Change {} of {}/{}", "v_ed", self.member_id, self.action_id), &format!("{} von {}/{} ändern", "v_ed", self.member_id, self.action_id)': '"Change characteristic shear force of action {}/{}", self.member_id, self.action_id), &format!("Charakteristische Querkraft der Einwirkung {}/{} ändern", self.member_id, self.action_id)',
    '"Change {} of {}/{}", "m_k", self.member_id, self.action_id), &format!("{} von {}/{} ändern", "m_k", self.member_id, self.action_id)': '"Change characteristic bending moment of action {}/{}", self.member_id, self.action_id), &format!("Charakteristisches Biegemoment der Einwirkung {}/{} ändern", self.member_id, self.action_id)',
    '"Change {} of {}/{}", "n_ed", self.member_id, self.action_id), &format!("{} von {}/{} ändern", "n_ed", self.member_id, self.action_id)': '"Change characteristic axial force of action {}/{}", self.member_id, self.action_id), &format!("Charakteristische Normalkraft der Einwirkung {}/{} ändern", self.member_id, self.action_id)',
    '"Change {} of {}", "h_ef", self.anchor_id), &format!("{} von {} ändern", "h_ef", self.anchor_id)': '"Change effective embedment depth of anchor {}", self.anchor_id), &format!("Effektive Verankerungstiefe von Dübel {} ändern", self.anchor_id)',
    '"Change {} of {}", "a_s", self.anchor_id), &format!("{} von {} ändern", "a_s", self.anchor_id)': '"Change stressed cross-section of anchor {}", self.anchor_id), &format!("Spannungsquerschnitt von Dübel {} ändern", self.anchor_id)',
    '"Change title to {}", self.new_title), &format!("title auf {} ändern", self.new_title)': '"Change report title to {}", self.new_title), &format!("Berichtstitel auf {} ändern", self.new_title)',
    '"Change design-working-life to {}", self.new_years), &format!("design-working-life auf {} ändern", self.new_years)': '"Change design working life to {} years", self.new_years), &format!("Geplante Nutzungsdauer auf {} Jahre ändern", self.new_years)',
    '"Change delta-c-dev to {}", self.new_delta_c_dev), &format!("delta-c-dev auf {} ändern", self.new_delta_c_dev)': '"Change cover allowance for deviation Δc,dev to {}", self.new_delta_c_dev), &format!("Vorhaltemaß Δc,dev auf {} ändern", self.new_delta_c_dev)',
    '"Change cement-type to {}", self.new_cement_type), &format!("cement-type auf {} ändern", self.new_cement_type)': '"Change cement type to {}", self.new_cement_type), &format!("Zementart auf {} ändern", self.new_cement_type)',
    '"Change annex to {:?}", self.new_annex), &format!("Anhang auf {:?} ändern", self.new_annex)': '"Change national annex to {:?}", self.new_annex), &format!("Nationalen Anhang auf {:?} ändern", self.new_annex)',
    '"Change grade {} strength", self.grade_id), &format!("Festigkeit von Sorte {} ändern", self.grade_id)': '"Change strength of grade {}", self.grade_id), &format!("Festigkeit der Sorte {} ändern", self.grade_id)',
    '"Change fire axis distance of {}", self.member_id), &format!("Achsabstand Brandschutz von {} ändern", self.member_id)': '"Change fire axis distance of member {}", self.member_id), &format!("Achsabstand für den Brandschutz von Bauteil {} ändern", self.member_id)',
    '"Change fire rating of {}", self.member_id), &format!("Feuerwiderstand von {} ändern", self.member_id)': '"Change fire resistance of member {}", self.member_id), &format!("Feuerwiderstand von Bauteil {} ändern", self.member_id)',
    '"Change exposure of {}", self.member_id), &format!("Exposition von {} ändern", self.member_id)': '"Change exposure class of member {}", self.member_id), &format!("Expositionsklasse von Bauteil {} ändern", self.member_id)',
    '"Change stirrup spacing of {}", self.member_id), &format!("Bügelabstand von {} ändern", self.member_id)': '"Change stirrup spacing of member {}", self.member_id), &format!("Bügelabstand von Bauteil {} ändern", self.member_id)',
}
EN1993 = {"member": ("member", "Bauteil"), "pile": ("pile", "Pfahl"), "tension-component": ("tension component", "Zugglied"), "joint": ("joint", "Anschluss"), "tower-leg": ("tower leg", "Turmstiel"), "cold-formed-member": ("cold-formed member", "Kaltprofil-Bauteil"), "load-case": ("load case", "Lastfall"), "material": ("material", "Werkstoff"), "fatigue-detail": ("fatigue detail", "Kerbdetail"), "bridge-fatigue": ("bridge fatigue verification", "Ermüdungsnachweis der Brücke"), "crane-runway": ("crane runway", "Kranbahn"), "section": ("cross-section", "Querschnitt"), "silo-shell": ("silo shell", "Siloschale"), "fire-exposure": ("fire exposure", "Brandbeanspruchung"), "plated-panel": ("plated panel", "Beulfeld"), "member-action": ("member action", "Bauteilbeanspruchung")}
UPSERT = {"member": "Bauteil", "joint": "Anschluss", "tower leg": "Turmstiel", "load case": "Lastfall", "crane runway": "Kranbahn", "section": "Querschnitt", "fire exposure": "Brandbeanspruchung", "silo shell": "Siloschale", "cold-formed member": "Kaltprofil-Bauteil", "pile": "Pfahl", "bridge fatigue": "Ermüdungsnachweis der Brücke", "member action": "Bauteilbeanspruchung", "fatigue detail": "Kerbdetail", "material": "Werkstoff", "tension component": "Zugglied", "plated panel": "Beulfeld"}
ISO = {"productSeries": ("product series", "Produktserie"), "productClass": ("product class", "Produktklasse"), "productIndex": ("product index", "Produktindex")}
VDI = {
    '"Change correction-as-of to {}-{:02}"': '"Change correction date to {}-{:02}"',
    '"Update security limits (max-file-bytes={})"': '"Update security limits (maximum file size {} bytes)"',
    '"Sicherheitsgrenzwerte (Max-Datei-Bytes={}) aktualisieren"': '"Sicherheitsgrenzwerte (maximale Dateigröße {} Byte) aktualisieren"',
}


def en1993(text):
    for slug, (en, de) in EN1993.items():
        text = text.replace(f'"Remove {slug} #{{}}"', f'"Remove {en} #{{}}"').replace(f'"{slug} #{{}} entfernen"', f'"{de} #{{}} entfernen"')
        text = text.replace(f'"Insert {slug} at #{{}}"', f'"Insert {en} at position #{{}}"').replace(f'"{slug} an #{{}} einfügen"', f'"{de} an Position #{{}} einfügen"')
    for en, de in UPSERT.items():
        text = re.sub(rf'"Upsert {re.escape(en)} \{{\}}"', f'"Update {en} {{}}"', text)
        text = re.sub(r'"[A-ZÄÖÜ][^"]*? setzen \{\}"', lambda match: match.group(0), text)
    for de in set(UPSERT.values()) | {"Brückenermüdung", "Kaltprofil", "Ermüdungsdetail", "Beulblech"}:
        target = {"Brückenermüdung": "Ermüdungsnachweis der Brücke", "Kaltprofil": "Kaltprofil-Bauteil", "Ermüdungsdetail": "Kerbdetail", "Beulblech": "Beulfeld"}.get(de, de)
        text = text.replace(f'"{de} setzen {{}}"', f'"{target} {{}} aktualisieren"')
    return text


def iso(text):
    for token, (en, de) in ISO.items():
        text = text.replace(f'"Introduce {token} \\"{{}}\\""', f'"Introduce {en} \\"{{}}\\""').replace(f'"Retire {token} \\"{{}}\\""', f'"Retire {en} \\"{{}}\\""')
        text = text.replace(f'"{token} \\"{{}}\\" erstellen"', f'"{de} \\"{{}}\\" einführen"').replace(f'"{token} \\"{{}}\\" löschen"', f'"{de} \\"{{}}\\" zurückziehen"')
    return text


changed = 0
for family, rewrite in (("🏛️en1992", lambda t: [t := t.replace(a, b) for a, b in EN1992.items()][-1]), ("🔩️en1993", en1993), ("📇️iso16757", iso), ("🏭️vdi3805", lambda t: [t := t.replace(a, b) for a, b in VDI.items()][-1])):
    for path in glob.glob(f"{ROOT}/{family}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/🦀️.rs") + glob.glob(f"{ROOT}/{family}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/🦠️mutation/🦀️.rs"):
        text = open(path, encoding="utf-8").read()
        if "fn label(&self)" not in text:
            continue
        new = rewrite(text)
        if new != text:
            changed += 1
            if write:
                open(path, "w", encoding="utf-8").write(new)
print(f"{changed} files {'rewritten' if write else 'to rewrite'}")
