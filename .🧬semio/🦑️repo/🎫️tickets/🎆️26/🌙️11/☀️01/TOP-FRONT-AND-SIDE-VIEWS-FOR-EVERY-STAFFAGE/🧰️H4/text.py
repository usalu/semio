"""Decision texts (English and German) of the front and plan views, per asset and scale."""

STD = {
    "standing-adult": dict(
        top=("hoodie", "Sweatshirt"), legs=("jeans", "Jeans"), shoes=("sneakers", "Sneaker"),
        d3=("hem rib, kangaroo pocket, drawstrings, collar and the hood bunched around the neck", "Saumbund, Känguru-Tasche, Kordeln, Kragen und der um den Hals gebauschten Kapuze"),
        d2=("hem line, pocket, collar and hood", "Saumlinie, Tasche, Kragen und Kapuze"),
        x3=("", ""), p3=(" and the hood behind the neck", " und die Kapuze hinter dem Hals"),
    ),
    "standing-phone-user": dict(
        top=("hoodie", "Sweatshirt"), legs=("jeans", "Jeans"), shoes=("sneakers", "Sneaker"),
        d3=("hem rib, kangaroo pocket, drawstrings and collar", "Saumbund, Känguru-Tasche, Kordeln und Kragen"),
        d2=("hem line, pocket and collar", "Saumlinie, Tasche und Kragen"),
        x3=(", the headphones hanging at the neck and the phone held in the right hand", ", die um den Hals hängenden Kopfhörer und das Telefon in der rechten Hand"),
        p3=(" and the phone in the right hand", " und das Telefon in der rechten Hand"),
    ),
    "walking-commuter": dict(
        top=("wool coat", "Wollmantel"), legs=("trousers", "Hose"), shoes=("leather shoes", "Lederschuhe"),
        d3=("front edge, hem, pockets and three buttons, the scarf knot", "Vorderkante, Saum, Taschen und drei Knöpfe, den Schalknoten"),
        d2=("front edge, hem and pockets, the scarf", "Vorderkante, Saum und Taschen, den Schal"),
        x3=(" and the shoulder bag with strap and flap hanging on the left", " und die links hängende Umhängetasche mit Riemen und Klappe"),
        p3=(" and the bag at the left side", " und die Tasche an der linken Seite"),
    ),
    "tour-guide": dict(
        top=("softshell jacket", "Softshell-Jacke"), legs=("trousers", "Hose"), shoes=("shoes", "Schuhe"),
        d3=("zip, hem, lanyard and badge", "Reißverschluss, Saum, Umhängeband und Namensschild"),
        d2=("zip and hem", "Reißverschluss und Saum"),
        x3=(", the cap, the pointer in the right hand and the flag pole with its flag seen edge-on in the left hand", ", die Mütze, den Zeigestab in der rechten Hand und die Fahnenstange mit der von der Kante gesehenen Fahne in der linken Hand"),
        p3=(", the cap, the pointer, the pole and the flag", ", die Mütze, den Zeigestab, die Stange und die Fahne"),
    ),
    "worker-carrying-box": dict(
        top=("work shirt", "Arbeitshemd"), legs=("work trousers", "Arbeitshose"), shoes=("boots", "Arbeitsschuhe"),
        d3=("hi-vis vest with two reflective bands, the helmet with ridge and the knee pad", "Warnweste mit zwei Reflexstreifen, den Helm mit Grat und den Knieschoner"),
        d2=("hi-vis vest with its bands and the helmet", "Warnweste mit Streifen und den Helm"),
        x3=(" and the cardboard box with tape and handle held at chest height", " und den Karton mit Klebeband und Griff auf Brusthöhe"),
        p3=(" and the cardboard box with its tape", " und den Karton mit Klebeband"),
    ),
}


def std_texts(aid, sc):
    t = STD[aid]
    top, legs, shoes = t["top"], t["legs"], t["shoes"]
    if sc == "50":
        f_en = f"Front view (the figure's left is on the right of the sheet): head with hair, ears, eyes, brows, nose and mouth, neck, {top[0]} with {t['d3'][0]}, sleeves with cuffs, mitten hands with finger lines, {legs[0]} with knee folds and hems, {shoes[0]} with sole band and laces{t['x3'][0]}. Plan: shoulders with seams, waist and trunk, head with hair parting, face, nose and ears, upper arms, forearms, hands, thighs, shanks and {shoes[0]} with laces{t['p3'][0]}. Limbs are bound to their own bones: strides, raised arms and bent knees shorten or stretch the shapes with the pose."
        f_de = f"Vorderansicht (die linke Körperseite liegt rechts im Blatt): Kopf mit Haar, Ohren, Augen, Brauen, Nase und Mund, Hals, {top[1]} mit {t['d3'][1]}, Ärmel mit Bündchen, Fäustlingshände mit Fingerlinien, {legs[1]} mit Kniefalten und Säumen, {shoes[1]} mit Sohlenband und Schnürung{t['x3'][1]}. Grundriss: Schultern mit Nähten, Taille und Rumpf, Kopf mit Scheitel, Gesicht, Nase und Ohren, Oberarme, Unterarme, Hände, Oberschenkel, Unterschenkel und {shoes[1]} mit Schnürung{t['p3'][1]}. Die Glieder sind an ihre eigenen Knochen gebunden: Schritte, erhobene Arme und gebeugte Knie verkürzen oder strecken die Formen mit der Pose."
        ov_en = "Front and plan: every filled shape carries the region of the garment or body part it depicts, so thermal, structural and transport overlays read in all three views."
        ov_de = "Vorderansicht und Grundriss: Jede gefüllte Form trägt die Region des dargestellten Kleidungsstücks oder Körperteils, sodass Wärme-, Tragwerks- und Transportüberlagerungen in allen drei Ansichten lesbar sind."
        return {"silhouette": (f_en, f_de), "overlays": (ov_en, ov_de)}
    if sc == "100":
        f_en = f"Front view (the figure's left is on the right of the sheet): head with hair and ears, neck, {top[0]} with {t['d2'][0]}, sleeves with cuff lines, mitten hands, {legs[0]} with hem lines, {shoes[0]} with sole lines{t['x3'][0]}. Plan: shoulders, waist, trunk, head with face, arms with hands, legs and {shoes[0]}{t['p3'][0]}; no ears, nose, parting or laces. Limbs are bound to their own bones and stretch with the pose."
        f_de = f"Vorderansicht (die linke Körperseite liegt rechts im Blatt): Kopf mit Haar und Ohren, Hals, {top[1]} mit {t['d2'][1]}, Ärmel mit Bündchenlinien, Fäustlingshände, {legs[1]} mit Saumlinien, {shoes[1]} mit Sohlenlinien{t['x3'][1]}. Grundriss: Schultern, Taille, Rumpf, Kopf mit Gesicht, Arme mit Händen, Beine und {shoes[1]}{t['p3'][1]}; ohne Ohren, Nase, Scheitel und Schnürung. Die Glieder sind an ihre eigenen Knochen gebunden und dehnen sich mit der Pose."
        ov_en = "Front and plan: regions as in the side view; details inherit the region of the garment they sit on."
        ov_de = "Vorderansicht und Grundriss: Regionen wie in der Seitenansicht; Details erben die Region des Kleidungsstücks, auf dem sie sitzen."
        return {"silhouette": (f_en, f_de), "overlays": (ov_en, ov_de)}
    if sc == "200":
        f_en = f"Front view (the figure's left is on the right of the sheet): plain masses of head with hair, neck, {top[0]}, sleeves, forearms, hands, thighs, shanks and {shoes[0]}{t['x3'][0]}; the legs keep a gap of more than 30 mm real size, the hands are wider than 40 mm. Plan: {shoes[0]}, legs, arms, hands, trunk, shoulders and head as main masses{t['p3'][0]}; no face details."
        f_de = f"Vorderansicht (die linke Körperseite liegt rechts im Blatt): schlichte Massen aus Kopf mit Haar, Hals, {top[1]}, Ärmeln, Unterarmen, Händen, Ober- und Unterschenkeln und {shoes[1]}{t['x3'][1]}; die Beine behalten eine Lücke von mehr als 30 mm in Wirklichkeit, die Hände sind breiter als 40 mm. Grundriss: {shoes[1]}, Beine, Arme, Hände, Rumpf, Schultern und Kopf als Hauptmassen{t['p3'][1]}; ohne Gesichtseinzelheiten."
        ov_en = "Front and plan: regions as in the side view; the hair mass carries the hair region and the face the skin region."
        ov_de = "Vorderansicht und Grundriss: Regionen wie in der Seitenansicht; die Haarmasse trägt die Haar-, das Gesicht die Hautregion."
        return {"silhouette": (f_en, f_de), "overlays": (ov_en, ov_de)}
    f_en = f"Front view (the figure's left is on the right of the sheet): one composed silhouette of head, torso with shoulders, two arms and two overlapping legs with feet; the hands merge into the forearms and the ankles are held at the minimum feature width of 0.10 m{t['x3'][0].replace(' held', ' hanging')}. Plan: {shoes[0]}, legs, arms, trunk with shoulders and head as a few merged masses{t['p3'][0]}."
    f_de = f"Vorderansicht (die linke Körperseite liegt rechts im Blatt): eine komponierte Silhouette aus Kopf, Rumpf mit Schultern, zwei Armen und zwei überlappenden Beinen mit Füßen; die Hände gehen in die Unterarme auf, die Knöchel bleiben auf der Mindesteinzelheit von 0,10 m{t['x3'][1]}. Grundriss: {shoes[1]}, Beine, Arme, Rumpf mit Schultern und Kopf als wenige verschmolzene Massen{t['p3'][1]}."
    ov_en = "Front and plan: merged shapes list all regions they show (for example the forearm with its hand)."
    ov_de = "Vorderansicht und Grundriss: Verschmolzene Formen führen alle Regionen auf, die sie zeigen (etwa der Unterarm mit seiner Hand)."
    return {"silhouette": (f_en, f_de), "overlays": (ov_en, ov_de)}


SPECIAL = {
    "wheelchair-user": {
        "50": ("Front view (the user's left is on the right of the sheet): head with beanie, ears, eyes and brows, neck, hoodie with hood, sleeves with cuffs, gloved hands with finger lines, knees and shanks in jeans, sneakers on the footplate, seat cushion and back sling, push handles, back tubes, hangers, rear tyres with hand rims and hubs seen edge-on, castors and brake levers. Plan: head, shoulders, trunk, arms, thighs, shanks and shoes above the seat cushion, frame rails, push handles, footplate, castors and both rear tyres with hand rims.", "Vorderansicht (die linke Seite der Person liegt rechts im Blatt): Kopf mit Mütze, Ohren, Augen und Brauen, Hals, Kapuzenpullover mit Kapuze, Ärmel mit Bündchen, Handschuhhände mit Fingerlinien, Knie und Unterschenkel in Jeans, Sneaker auf der Fußplatte, Sitzkissen und Rückenbespannung, Schiebegriffe, Rückenrohre, Frontrohre, Hinterreifen mit Greifreifen und Naben von der Kante gesehen, Lenkrollen und Bremshebel. Grundriss: Kopf, Schultern, Rumpf, Arme, Oberschenkel, Unterschenkel und Schuhe über dem Sitzkissen, Rahmenrohre, Schiebegriffe, Fußplatte, Lenkrollen und beide Hinterreifen mit Greifreifen."),
        "100": ("Front view: head with beanie and ears, neck, hoodie with hood, sleeves, gloved hands, knees and shanks in jeans, sneakers, seat cushion, back sling, push handles, back tubes, hangers, rear tyres with hand rims and castors. Plan: the user above seat cushion, frame rails, push handles, footplate, castors and rear tyres with hand rims; hubs, brake levers and eyes are not drawn.", "Vorderansicht: Kopf mit Mütze und Ohren, Hals, Kapuzenpullover mit Kapuze, Ärmel, Handschuhhände, Knie und Unterschenkel in Jeans, Sneaker, Sitzkissen, Rückenbespannung, Schiebegriffe, Rückenrohre, Frontrohre, Hinterreifen mit Greifreifen und Lenkrollen. Grundriss: die Person über Sitzkissen, Rahmenrohren, Schiebegriffen, Fußplatte, Lenkrollen und Hinterreifen mit Greifreifen; Naben, Bremshebel und Augen werden nicht gezeichnet."),
        "200": ("Front view: plain masses of head with beanie, neck, hoodie, sleeves, gloved hands, knees, shanks, sneakers, seat cushion, back sling, the two hangers, rear tyres seen edge-on (0.045 m, above the minimum feature) and castors. Plan: the user, seat cushion, footplate, rear tyres and castors; frame tubes thinner than 40 mm are omitted.", "Vorderansicht: schlichte Massen aus Kopf mit Mütze, Hals, Kapuzenpullover, Ärmeln, Handschuhhänden, Knien, Unterschenkeln, Sneakern, Sitzkissen, Rückenbespannung, den beiden Frontrohren, von der Kante gesehenen Hinterreifen (0,045 m, über der Mindesteinzelheit) und Lenkrollen. Grundriss: die Person, Sitzkissen, Fußplatte, Hinterreifen und Lenkrollen; Rahmenrohre unter 40 mm Breite entfallen."),
        "500": ("Front view: one silhouette of the user, a seat block, a footplate and each rear wheel as one 0.10 m wide mass (tyre and hand rim merged); castors, frame tubes and hubs are dropped. Plan: the user, seat block, footplate and the two merged wheels.", "Vorderansicht: eine Silhouette der Person, ein Sitzblock, eine Fußplatte und jedes Hinterrad als eine 0,10 m breite Masse (Reifen und Greifreifen verschmolzen); Lenkrollen, Rahmenrohre und Naben entfallen. Grundriss: die Person, Sitzblock, Fußplatte und die beiden verschmolzenen Räder."),
    },
    "sleeping-person": {
        "50": ("Front view (seen from the foot end, the sleeper's left is on the right of the sheet): pillow, head with hair, neck, the duvet over chest, hips, thighs and shins as stacked domes, the duvet end with hem, both sleeves and hands lying on the duvet and both bare soles. Plan: pillow, head with face, the duvet masses over trunk, hips, thighs and shins with fold line, the two arms with hands, the duvet skirts and the bare feet.", "Vorderansicht (vom Fußende gesehen, die linke Seite der schlafenden Person liegt rechts im Blatt): Kissen, Kopf mit Haar, Hals, die Bettdecke über Brust, Hüfte, Oberschenkeln und Schienbeinen als gestaffelte Kuppeln, das Deckenende mit Saum, beide Ärmel und Hände auf der Decke und beide nackten Fußsohlen. Grundriss: Kissen, Kopf mit Gesicht, die Deckenmassen über Rumpf, Hüfte, Oberschenkeln und Schienbeinen mit Faltenlinie, die beiden Arme mit Händen, die Deckenenden und die nackten Füße."),
        "100": ("Front view (from the foot end): pillow, head with hair, neck, duvet domes over chest, hips, thighs and shins, the duvet end with hem, sleeves and hands and both soles. Plan: pillow, head with face, duvet masses, arms with hands, duvet skirts and feet; the fold line is drawn only over the chest.", "Vorderansicht (vom Fußende): Kissen, Kopf mit Haar, Hals, Deckenkuppeln über Brust, Hüfte, Oberschenkeln und Schienbeinen, das Deckenende mit Saum, Ärmel und Hände und beide Fußsohlen. Grundriss: Kissen, Kopf mit Gesicht, Deckenmassen, Arme mit Händen, Deckenenden und Füße; die Faltenlinie nur über der Brust."),
        "200": ("Front view (from the foot end): pillow, head with hair, neck, duvet domes, duvet end, sleeves with hands and the bare soles as plain masses without fold lines. Plan: pillow, head, duvet masses, arms, duvet skirts and feet.", "Vorderansicht (vom Fußende): Kissen, Kopf mit Haar, Hals, Deckenkuppeln, Deckenende, Ärmel mit Händen und die nackten Fußsohlen als schlichte Massen ohne Faltenlinien. Grundriss: Kissen, Kopf, Deckenmassen, Arme, Deckenenden und Füße."),
        "500": ("Front view (from the foot end): pillow, head, three stacked duvet domes (chest, thighs, shins), the duvet end and both soles merged to one foot mass each; sleeves, hands and neck are merged into the duvet. Plan: pillow, head, three duvet masses, the duvet skirts and the feet.", "Vorderansicht (vom Fußende): Kissen, Kopf, drei gestaffelte Deckenkuppeln (Brust, Oberschenkel, Schienbeine), das Deckenende und beide Fußsohlen als je eine Fußmasse; Ärmel, Hände und Hals gehen in der Decke auf. Grundriss: Kissen, Kopf, drei Deckenmassen, die Deckenenden und die Füße."),
    },
}


def texts(aid, sc):
    if aid in STD:
        return std_texts(aid, sc)
    en, de = SPECIAL[aid][sc]
    return {"silhouette": (en, de), "overlays": ("Front and plan: every filled shape carries the region of the part it shows; merged shapes list all their regions.", "Vorderansicht und Grundriss: Jede gefüllte Form trägt die Region des dargestellten Teils; verschmolzene Formen führen alle ihre Regionen auf.")}
