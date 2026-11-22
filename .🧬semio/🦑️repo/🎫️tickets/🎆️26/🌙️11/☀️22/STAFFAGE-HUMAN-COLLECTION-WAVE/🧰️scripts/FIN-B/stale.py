"""🕵️ Finds texts of a derived human that still speak of a prop of its base figure: python3 stale.py   (scope and words are listed below; provenance notes may name the base)."""
import json
import re

HUMANS = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans"
WORDS = {
    "picnicker-sitting-on-ground": r"guitar|Gitarre|stool|Hocker|strum|fret",
    "person-eating-standing": r"phone|Telefon|Handy|headphone|Kopfhörer|screen|Bildschirm",
    "person-drinking-coffee": r"point|zeig|fann|fächel|shiver|zitter",
    "waving-person": r"shiver|zitter|fann|fächel",
    "photographer-with-camera": r"flag|Fahne|pointer|Zeigestab|\bpole\b|tour|Führung",
    "person-taking-selfie": r"headphone|Kopfhörer",
    "standing-violinist": r"shiver|zitter|fann|fächel",
    "painter-at-easel": r"\bcane\b|Stock\b|Gehstock|derby|Derby",
    "dancer": r"runner|Läufer|\brun\b|running|Lauf|jog",
    "person-sweeping": r"shiver|zitter|fann|fächel",
    "person-hanging-laundry": r"flag|Fahne|pointer|Zeigestab|tour|Führung|lanyard|badge|Ausweis",
    "person-carrying-shopping-bags": r"shoulder bag|Umhängetasche|strap|Riemen|watch|Uhr\b",
    "person-with-guide-dog": r"\bcane\b|Langstock|Stock\b|sweep|pendel",
    "person-signing": r"shiver|zitter|fann|fächel",
    "person-with-arm-sling": r"shoulder bag|Umhängetasche|bag|Tasche",
    "person-in-rain-poncho": r"shoulder bag|Umhängetasche|bag|Tasche|watch|Uhr\b|overcoat|scarf|Schal",
    "person-in-winter-coat": r"shoulder bag|Umhängetasche|\bbag|Tasche\b|watch|Uhr\b",
    "sunbather-on-towel": r"sweatshirt|jeans|Jeans|sneaker|Sneaker|hood|Kapuze|shiver|zitter",
    "person-queuing": r"shiver|zitter|fann|fächel|point|zeig",
    "person-checking-watch": r"phone|Telefon|Handy|Smartphone|screen|Bildschirm",
    "parent-with-child-on-shoulders": r"on the hip|auf der Hüfte|toddler on|carrier|Trage",
    "worker-with-pallet-jack": r"hand truck|Sackkarre|courier|Kurier|parcel|Paket",
    "person-carrying-ladder": r"carton|Karton|\bbox\b|Kiste",
    "postal-carrier": r"watch|Uhr\b|shoulder bag|Umhängetasche|leather|Leder",
}


def walk(x, path, pattern, out):
    """🚶️ Collects bilingual texts matching the pattern."""
    if isinstance(x, dict):
        if set(x) == {"en", "de"} and isinstance(x["en"], str):
            for lang in ("en", "de"):
                for sentence in re.split(r"(?<=[.;:]) ", x[lang]):
                    if re.search(pattern, sentence):
                        out.append((path, lang, sentence))
        else:
            for k, v in x.items():
                walk(v, f"{path}/{k}", pattern, out)
    elif isinstance(x, list):
        for i, v in enumerate(x):
            walk(v, f"{path}/{i}", pattern, out)


for slug, pattern in WORDS.items():
    try:
        a = json.load(open(f"{HUMANS}/{slug}/🔣️.json", encoding="utf8"))["asset"]
    except FileNotFoundError:
        continue
    out = []
    walk({k: v for k, v in a.items() if k != "provenance"}, "", pattern, out)
    print(f"== {slug}: {len(out)}")
    for path, lang, sentence in out[:40]:
        print("  ", path, lang, "|", sentence[:170])
