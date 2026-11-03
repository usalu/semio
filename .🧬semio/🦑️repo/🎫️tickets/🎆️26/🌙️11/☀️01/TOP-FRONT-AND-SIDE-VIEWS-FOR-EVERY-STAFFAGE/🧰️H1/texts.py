"""H1 scratch: decision sentences for the front and plan views, per scale (en, de). Asset phrases are appended per key."""

BASE = {
    500: dict(
        silhouette=(
            "Front and plan are each one composed silhouette designed for the paper size, not scaled down from a larger scale: head, shoulders with trunk and hips, one thickened shape per arm and per leg. Every shape sits in rest-pose coordinates on the same bones as the side view, so limbs stretch in plan and rise or shorten in front with every pose and clip.",
            "Vorder- und Draufsicht sind je eine zusammengesetzte Silhouette, für das Papierformat entworfen und nicht von einem größeren Maßstab verkleinert: Kopf, Schultern mit Rumpf und Hüfte, je eine verdickte Form für jeden Arm und jedes Bein. Alle Formen liegen in Ruhehaltungs-Koordinaten an denselben Knochen wie die Seitenansicht; so strecken sich die Glieder im Grundriss und heben oder verkürzen sich in der Vorderansicht bei jeder Haltung und jedem Clip.",
        ),
        retained=(
            "Retained: head, shoulders, hips, each arm and each leg as one closed shape; the plan keeps one stadium per limb segment with the stride length of the legs.",
            "Beibehalten: Kopf, Schultern, Hüfte, jeder Arm und jedes Bein als eine geschlossene Form; der Grundriss behält je Gliedsegment ein Stadion mit der Schrittlänge der Beine.",
        ),
        merged=(
            "Merged: head and hair share one shape (hair and face regions listed together); clothing layers are not separated; limbs are not thinner than the printable width of 0.1 m; legs and trunk touch where the profile gap would be smaller than the gap limit.",
            "Zusammengefasst: Kopf und Haar bilden eine Form (Haar- und Gesichtsregion gemeinsam); Kleidungsschichten sind nicht getrennt; Glieder sind nicht dünner als die druckbare Breite von 0,1 m; Beine und Rumpf berühren sich dort, wo der Zwischenraum unter der Lückengrenze läge.",
        ),
        omitted=(
            "Omitted: neck, knees, face, ears, hair line and every seam; hands and toes are single blunt stadiums.",
            "Weggelassen: Hals, Knie, Gesicht, Ohren, Haaransatz und alle Nähte; Hände und Zehen sind einfache stumpfe Stadien.",
        ),
        thin="Thin parts are thickened only to the printable limit (about 0.1 m of model width) or dropped; they are not hatched.",
        thin_de="Dünne Teile sind nur bis zur druckbaren Grenze (etwa 0,1 m Modellbreite) verdickt oder entfallen; sie sind nicht schraffiert.",
        overlays=(
            "The merged head and shoulder shapes list all their regions, so thermal, structural and transport overlays are painted area-weighted from the same regions as the side view.",
            "Die zusammengefassten Kopf- und Schulterformen führen alle ihre Regionen auf; Thermik-, Tragwerks- und Transportüberlagerungen werden flächengewichtet aus denselben Regionen wie in der Seitenansicht gemalt.",
        ),
    ),
    200: dict(
        silhouette=(
            "Front and plan show the main masses designed for this scale: head with hair, torso, pelvis, and separate arm, forearm, hand, thigh, shank and shoe masses; the knee is an extra form in front. The shapes sit on the same bones as the side view, so strides stretch the limbs in plan and bending knees rise in front.",
            "Vorder- und Draufsicht zeigen die für diesen Maßstab entworfenen Hauptmassen: Kopf mit Haar, Rumpf, Becken sowie getrennte Massen für Oberarm, Unterarm, Hand, Oberschenkel, Unterschenkel und Schuh; das Knie ist in der Vorderansicht eine zusätzliche Form. Die Formen liegen an denselben Knochen wie die Seitenansicht; Schritte strecken die Glieder im Grundriss, gebeugte Knie heben sich in der Vorderansicht.",
        ),
        retained=(
            "Retained: every limb segment per side, the shoulder width and hip width of the real figure, face and hair as two regions.",
            "Beibehalten: jedes Gliedsegment je Seite, die Schulter- und Hüftbreite der echten Figur, Gesicht und Haar als zwei Regionen.",
        ),
        merged=(
            "Merged: clothing layers of one segment into one shape; toe box and shoe into the foot silhouette where they touch.",
            "Zusammengefasst: Kleidungsschichten eines Segments zu einer Form; Schuhkappe und Schuh zur Fußsilhouette, wo sie sich berühren.",
        ),
        omitted=(
            "Omitted: ears, nose, hair line, seams, laces and every fold.",
            "Weggelassen: Ohren, Nase, Haaransatz, Nähte, Schnürung und jede Falte.",
        ),
        thin="Thin parts such as fingers are not drawn; the hand is one mass.",
        thin_de="Dünne Teile wie Finger sind nicht gezeichnet; die Hand ist eine Masse.",
        overlays=(
            "Every filled shape carries the region of its side-view counterpart, so the overlays read the same surfaces in front and plan.",
            "Jede gefüllte Form trägt die Region ihres Gegenstücks in der Seitenansicht; die Überlagerungen lesen daher in Vorderansicht und Grundriss dieselben Flächen.",
        ),
    ),
    100: dict(
        silhouette=(
            "Front and plan are articulated outlines designed for this scale: clothing profiles with calf and thigh swell, shoulders, mitten hands, shoe and toe box, hair mass, ears and the neck. Limb shapes follow their bones by travel only (plan: stadiums stretched from bone origin to tip; front: outlines stretched between origin and tip height), so every pose and clip reads correctly.",
            "Vorder- und Draufsicht sind für diesen Maßstab entworfene gegliederte Umrisse: Kleidungsprofile mit Waden- und Oberschenkelschwellung, Schultern, Fausthandschuhformen, Schuh mit Kappe, Haarmasse, Ohren und Hals. Gliedformen folgen ihren Knochen nur über den Weg (Grundriss: von Knochenursprung zu Spitze gestreckte Stadien; Vorderansicht: zwischen Ursprungs- und Spitzenhöhe gestreckte Umrisse); so lesen sich alle Haltungen und Clips richtig.",
        ),
        retained=(
            "Retained: all articulated segments, clothing silhouettes, hands, shoes, hair and the main clothing details that are wider than the line gap.",
            "Beibehalten: alle Gliedsegmente, Kleidungssilhouetten, Hände, Schuhe, Haar und die wichtigsten Kleidungsdetails, die breiter als die Linienlücke sind.",
        ),
        merged=(
            "Merged: skin and cloth layers that overlap by less than the gap limit; the toe box stays a separate form only where the toe bone moves.",
            "Zusammengefasst: Haut- und Stoffschichten, die sich weniger als die Lückengrenze überlappen; die Schuhkappe bleibt nur dort eine eigene Form, wo sich der Zehenknochen bewegt.",
        ),
        omitted=(
            "Omitted: individual fingers, facial features, stitching and fabric texture.",
            "Weggelassen: einzelne Finger, Gesichtszüge, Steppnähte und Stofftextur.",
        ),
        thin="Straps, bands and seams are drawn as detail lines of at least the detail pen only where they carry meaning for the figure; finer ones are omitted.",
        thin_de="Riemen, Bänder und Nähte sind nur dort als Detaillinien mit mindestens der Detailfeder gezeichnet, wo sie für die Figur Bedeutung tragen; feinere entfallen.",
        overlays=(
            "Each shape carries the region of its side-view counterpart; clothing and skin layers keep their own regions so layered thermal results are visible in all three views.",
            "Jede Form trägt die Region ihres Gegenstücks in der Seitenansicht; Kleidungs- und Hautschichten behalten ihre eigenen Regionen, sodass geschichtete Thermikergebnisse in allen drei Ansichten sichtbar sind.",
        ),
    ),
    50: dict(
        silhouette=(
            "Front and plan are drawn in full detail for this scale: tapered clothing outlines, hands with a thumb form, shoe construction, hair mass with parting, ears, face features in the front view and nose and ears in plan, plus the equipment of the figure. Every limb keeps its bone binding (plan: stadiums stretched along the bone; front: outlines stretched between the heights of bone origin and tip), so strides, bent knees and reaching arms read correctly in each pose and clip instant.",
            "Vorder- und Draufsicht sind für diesen Maßstab in voller Detaillierung gezeichnet: verjüngte Kleidungsumrisse, Hände mit Daumenform, Schuhaufbau, Haarmasse mit Scheitel, Ohren, Gesichtszüge in der Vorderansicht sowie Nase und Ohren im Grundriss, dazu die Ausrüstung der Figur. Jedes Glied behält seine Knochenbindung (Grundriss: entlang des Knochens gestreckte Stadien; Vorderansicht: zwischen den Höhen von Knochenursprung und Spitze gestreckte Umrisse); Schritte, gebeugte Knie und greifende Arme lesen sich daher in jeder Haltung und jedem Clip-Zeitpunkt richtig.",
        ),
        retained=(
            "Retained: every clothing layer with its own region, shoe details, hair, face and ear forms, equipment and the seams and folds that stay visible from the front or from above.",
            "Beibehalten: jede Kleidungsschicht mit eigener Region, Schuhdetails, Haar-, Gesichts- und Ohrenformen, Ausrüstung sowie die Nähte und Falten, die von vorn oder von oben sichtbar bleiben.",
        ),
        merged=(
            "Merged: only fabric texture and the finest stitching, which would fall below the detail pen.",
            "Zusammengefasst: nur Stofftextur und feinste Steppnähte, die unter die Detailfeder fielen.",
        ),
        omitted=(
            "Omitted: fingers as separate forms and anything hidden from the viewpoint.",
            "Weggelassen: Finger als eigene Formen und alles, was vom Blickpunkt aus verdeckt ist.",
        ),
        thin="Thin elements keep at least the detail pen; hatching is shared with the side view and used only where its spacing stays above the profile limit.",
        thin_de="Dünne Elemente behalten mindestens die Detailfeder; die Schraffuren sind mit der Seitenansicht geteilt und nur dort verwendet, wo ihr Abstand über der Profilgrenze bleibt.",
        overlays=(
            "Every filled shape carries the region and material of its side-view counterpart, so thermal, structural and transport views work from every viewpoint.",
            "Jede gefüllte Form trägt Region und Material ihres Gegenstücks in der Seitenansicht; Thermik-, Tragwerks- und Transportansichten funktionieren daher aus jedem Blickwinkel.",
        ),
    ),
}


def texts(L, extra=None):
    """Return (en, de) dicts for keys silhouette, retained, merged, omitted, thinElements, overlays; `extra[L][key] = (en, de)` is appended."""
    b = BASE[L]
    en, de = {}, {}
    for k in ("silhouette", "retained", "merged", "omitted", "overlays"):
        en[k], de[k] = b[k]
    en["thinElements"], de["thinElements"] = b["thin"], b["thin_de"]
    if extra and L in extra:
        for k, (e, d) in extra[L].items():
            en[k] = en[k] + " " + e
            de[k] = de[k] + " " + d
    return en, de
