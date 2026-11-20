/** 🗒️ The bilingual decision texts of the round-3 ivy on trellis: per scale the 3D decisions (body, thermal, structure) and the 2D decisions of the redrawn representations.
 * @see ./r3-ivy-on-trellis-model.ts */
type Text = { en: string; de: string };
export const T = (en: string, de: string): Text => ({ en, de });

export const DECISIONS_3D: Record<number, Text[]> = {
  50: [
    T(
      "Body: the trellis as built. Two planed 90 mm posts with chamfered heads stand in galvanised sleeves (4 mm wall, hollow) welded to 100 x 180 mm base plates with four hexagon anchor nuts and two through bolts each; two 70 mm rails between the posts; eight vertical laths (front layer) and seven horizontal laths (back layer) of 20 x 10 mm, each layer one solid with a repeat. Four main stems (20 mm at the collar, 7 mm at the tip) are planted 7 cm in front, pass the bottom rail in front and are woven through the lattice, alternately in front of and behind the horizontal laths; Each leaf mat is two cushions for the inner leaf layers; about 2200 single five-lobed leaves of 58 to 88 mm lie on them like roof tiles on both faces, in alternate rows along the green shoots that run out and up from every stem; leaves lie over the ridge, eight trailing shoots hang from the ragged lower edge and four shoots arch over the top rail. The sixteen woody laterals lie inside the mats and are modelled in the thermal and the structure list.",
      "Körper: das Spalier wie gebaut. Zwei gehobelte Pfosten von 90 mm mit gefasten Köpfen stehen in verzinkten Hülsen (4 mm Wand, hohl), die auf Fußplatten von 100 x 180 mm geschweißt sind, mit je vier Sechskant-Ankermuttern und zwei Durchgangsschrauben; zwei Riegel von 70 mm zwischen den Pfosten; acht senkrechte Latten (vordere Lage) und sieben waagrechte Latten (hintere Lage) von 20 x 10 mm, jede Lage ein Körper mit Wiederholung. Vier Haupttriebe (20 mm am Wurzelhals, 7 mm an der Spitze) sind 7 cm davor gepflanzt, laufen vor dem unteren Riegel vorbei und sind durch das Gitter geflochten, abwechselnd vor und hinter den waagrechten Latten; Jeder Blattteppich besteht aus zwei Polstern für die inneren Blattlagen; rund 2200 einzelne fünflappige Blätter von 58 bis 88 mm liegen wie Dachziegel auf beiden Seiten darauf, in wechselständigen Reihen entlang der grünen Triebe, die von jedem Haupttrieb nach außen und oben laufen; Blätter liegen über dem First, acht hängende Triebe fallen vom ausgefransten unteren Rand und vier Triebe biegen sich über den oberen Riegel. Die sechzehn verholzten Seitentriebe liegen in den Teppichen und sind in der thermischen und der Tragwerksliste modelliert.",
    ),
    T(
      "Thermal: the leaf regions are the mat cushions and every leaf by reference (upper mats and crown shoots, lower mats and trailing shoots); the hidden stem region is the four main stems and the sixteen laterals as their own solids (0.45 m2); timber references the body parts, the steel is the plates, nuts and bolts by reference and the sleeves as closed shells (their inner faces lie against the posts); the ten transpiration outlets are thin lenses lying on the front face of the mats at their anchors, on the crown masses on their top.",
      "Thermisch: die Blattregionen sind die Teppichpolster und jedes Blatt als Verweis (obere Teppiche und Kronentriebe, untere Teppiche und hängende Triebe); die verdeckte Triebregion sind die vier Haupttriebe und die sechzehn Seitentriebe als eigene Körper (0,45 m2); Holz verweist auf die Körperteile, der Stahl sind die Platten, Muttern und Schrauben als Verweis und die Hülsen als geschlossene Schalen (ihre Innenflächen liegen an den Pfosten); die zehn Transpirationsauslässe sind dünne Linsen auf der Vorderseite der Teppiche an ihren Ankern, auf den Kronenmassen auf deren Oberseite.",
    ),
    T(
      "Structure: the load path of the screen. Posts as members clamped in the sleeves (joints) on the base plates (fixed supports); rails and both lath layers as the panel members with a joint block at each rail end; the main stems as members with a collar joint at the ground and two ties to the lattice each; the laterals as the members of the leaf-mat bones with a joint where they leave the stem; the overhanging shoots as the crown members; point masses of the posts and the panel at their centres of mass.",
      "Tragwerk: der Lastpfad der Wand. Pfosten als Stäbe, in den Hülsen eingespannt (Knoten), auf den Fußplatten (feste Auflager); Riegel und beide Lattenlagen als Stäbe des Gitterfelds mit einem Knotenklotz an jedem Riegelende; die Haupttriebe als Stäbe mit einem Wurzelhalsknoten am Boden und je zwei Bindungen ans Gitter; die Seitentriebe als Stäbe der Blattteppichknochen mit einem Knoten am Abgang vom Haupttrieb; die überhängenden Triebe als Kronenstäbe; Punktmassen der Pfosten und des Gitterfelds in ihren Schwerpunkten.",
    ),
  ],
  100: [
    T(
      "Body: posts as plain square bars (the 15 mm head chamfer is at the size limit and omitted), hollow sleeves on their base plates without nuts and bolts, rails, the two lath layers at their true 20 x 10 mm as slender parts; the stems run as five-station tubes in front of the lattice (the weave is omitted); each mat is one cushion, the crown shoots one mass each; about 1070 three-lobed leaves, drawn a quarter larger so that each stands for two, lie on both faces and over the ridge.",
      "Körper: Pfosten als einfache Vierkantstäbe (die 15-mm-Kopffase liegt an der Größengrenze und entfällt), hohle Hülsen auf ihren Fußplatten ohne Muttern und Schrauben, Riegel, die zwei Lattenlagen in ihrer wahren Größe von 20 x 10 mm als schlanke Teile; die Triebe laufen als Rohre mit fünf Stationen vor dem Gitter (das Flechten entfällt); jeder Teppich ist ein Polster, die Kronentriebe je eine Masse; rund 1070 dreilappige Blätter, ein Viertel größer gezeichnet, sodass jedes für zwei steht, liegen auf beiden Seiten und über dem First.",
    ),
    T(
      "Thermal: mats and leaves by reference; stems as their own tubes with all sixteen laterals as three-station tubes (0.45 m2 kept); timber and steel by reference; ten outlet lenses on the mats.",
      "Thermisch: Teppiche und Blätter als Verweis; Triebe als eigene Rohre mit allen sechzehn Seitentrieben als Rohre mit drei Stationen (0,45 m2 erhalten); Holz und Stahl als Verweis; zehn Auslasslinsen auf den Teppichen.",
    ),
    T(
      "Structure: posts, sleeves, base plates, rails and lath layers as at 1:50; the stems by reference; one lateral per leaf-mat bone with its joint on the stem; one overhanging shoot per crown bone; the ties and the collar joints are omitted; point masses.",
      "Tragwerk: Pfosten, Hülsen, Fußplatten, Riegel und Lattenlagen wie bei 1:50; die Triebe als Verweis; ein Seitentrieb je Blattteppichknochen mit seinem Knoten am Haupttrieb; ein überhängender Trieb je Kronenknochen; Bindungen und Wurzelhalsknoten entfallen; Punktmassen.",
    ),
  ],
  200: [
    T(
      "Body: posts, sleeves without plates, rails, the four stems as slender tubes at their true size and eight mat cushions, one per leaf-mat bone, covered on both faces by about 240 leaf sprays: leaf-shaped tapered flakes of 0.2 m, one smallest feature thick, each standing for a group of leaves; the overhanging shoots are merged into the upper cushions; laths (0.1 mm on paper) are not part of the body.",
      "Körper: Pfosten, Hülsen ohne Platten, Riegel, die vier Triebe als schlanke Rohre in wahrer Größe und acht Teppichpolster, eines je Blattteppichknochen, auf beiden Seiten von rund 240 Blattbüscheln gedeckt: blattförmige, verjüngte Schuppen von 0,2 m, ein kleinstes Merkmal dick, jede für eine Gruppe von Blättern; die überhängenden Triebe sind in die oberen Polster eingeschmolzen; Latten (0,1 mm auf dem Papier) gehören nicht zum Körper.",
    ),
    T(
      "Thermal: the eight cushions and the leaf sprays by reference; stems as slender tubes at their true size with one lateral per bone; the timber as posts, rails and the two slender lath layers so that its surface stays honest; sleeves by reference; eight outlet lenses.",
      "Thermisch: die acht Polster und die Blattbüschel als Verweis; Triebe als schlanke Rohre in wahrer Größe mit einem Seitentrieb je Knochen; das Holz als Pfosten, Riegel und die zwei schlanken Lattenlagen, damit seine Oberfläche ehrlich bleibt; Hülsen als Verweis; acht Auslasslinsen.",
    ),
    T(
      "Structure: posts on the sleeves as supports, rails with a joint block at each end, four stems, one lateral per leaf-mat bone, one shoot per crown bone, point masses.",
      "Tragwerk: Pfosten auf den Hülsen als Auflager, Riegel mit einem Knotenklotz an jedem Ende, vier Triebe, ein Seitentrieb je Blattteppichknochen, ein Trieb je Kronenknochen, Punktmassen.",
    ),
  ],
  500: [
    T(
      "Body: two posts, the two rails, the four stems as slender tubes and the leaf screen as one thin mass between the posts, covered on both faces by about 130 leaf sprays (leaf-shaped tapered flakes of 0.33 m, 76 mm thick) that give the ragged outline and the full depth of 0.25 m; the open strip above the bottom rail stays open (0.7 mm on paper).",
      "Körper: zwei Pfosten, die zwei Riegel, die vier Triebe als schlanke Rohre und die Blattwand als eine dünne Masse zwischen den Pfosten, auf beiden Seiten von rund 130 Blattbüscheln gedeckt (blattförmige, verjüngte Schuppen von 0,33 m, 76 mm dick), die den ausgefransten Umriss und die volle Tiefe von 0,25 m geben; der offene Streifen über dem unteren Riegel bleibt offen (0,7 mm auf dem Papier).",
    ),
    T(
      "Thermal: the screen cut at 1.3 m into the upper and the lower leaf region, the sprays by reference in the region of their height; four slender stems; posts and rails by reference; the sleeves as closed steel shells; one outlet lens per emitter.",
      "Thermisch: die Wand bei 1,3 m in die obere und die untere Blattregion geschnitten, die Büschel als Verweis in der Region ihrer Höhe; vier schlanke Triebe; Pfosten und Riegel als Verweis; die Hülsen als geschlossene Stahlschalen; eine Auslasslinse je Emitter.",
    ),
    T(
      "Structure: posts on support blocks of the smallest feature, rails, four stems, one lateral per leaf-mat bone and one shoot per crown bone as slender tubes, point masses.",
      "Tragwerk: Pfosten auf Auflagerklötzen der kleinsten Merkmalsgröße, Riegel, vier Triebe, ein Seitentrieb je Blattteppichknochen und ein Trieb je Kronenknochen als schlanke Rohre, Punktmassen.",
    ),
  ],
};

export const DECISIONS_2D: Record<number, Record<string, Text>> = {
  50: {
    silhouette: T(
      "At 1:50 (1 m is 20 mm) the trellis is drawn as built: posts with chamfered heads in steel sleeves on base plates with nuts and bolts, two rails, eight vertical and seven horizontal 20 mm laths, the four stems as tapering filled outlines climbing in front of the bottom rail into the lattice; the leaf mats are the outlines of their cushions with single five-lobed leaves on them, trailing shoots below and shoots over the top rail. Front and plan: the front shows the 0.25 m thick mats beside the post, the base plate and the stems planted 7 cm in front; the plan shows the mats over the top rail, the post heads and the base plates.",
      "Bei 1:50 (1 m sind 20 mm) ist das Spalier wie gebaut gezeichnet: Pfosten mit gefasten Köpfen in Stahlhülsen auf Fußplatten mit Muttern und Schrauben, zwei Riegel, acht senkrechte und sieben waagrechte Latten von 20 mm, die vier Triebe als sich verjüngende gefüllte Umrisse, die vor dem unteren Riegel ins Gitter klettern; die Blattteppiche sind die Umrisse ihrer Polster mit einzelnen fünflappigen Blättern darauf, hängenden Trieben darunter und Trieben über dem oberen Riegel. Vorder- und Draufsicht: die Vorderansicht zeigt die 0,25 m dicken Teppiche neben dem Pfosten, die Fußplatte und die 7 cm davor gepflanzten Triebe; die Draufsicht zeigt die Teppiche über dem oberen Riegel, die Pfostenköpfe und die Fußplatten.",
    ),
    retained: T("Retained: every lath, posts, rails, sleeves, plates, nuts and bolt heads, the stems, the mat outlines and every third leaf of the front face.", "Beibehalten: jede Latte, Pfosten, Riegel, Hülsen, Platten, Muttern und Schraubenköpfe, die Triebe, die Teppichumrisse und jedes dritte Blatt der Vorderseite."),
    merged: T("Merged: the inner leaf layers of each mat as one outline of its cushions, drawn in its own field between the seams of the bones; in the plan the lower mats lie hidden under the upper ones.", "Zusammengefasst: die inneren Blattlagen jedes Teppichs als ein Umriss seiner Polster, in seinem eigenen Feld zwischen den Fugen der Knochen gezeichnet; in der Draufsicht liegen die unteren Teppiche verdeckt unter den oberen."),
    omitted: T("Omitted: leaf veins, petioles, the green leaf shoots of 3 mm, adhesive roots, the woody laterals under the leaves.", "Weggelassen: Blattadern, Blattstiele, die grünen Blatttriebe von 3 mm, Haftwurzeln, die verholzten Seitentriebe unter den Blättern."),
    thinElements: T("Trailing and overhanging shoots (5 to 7 mm) are strokes of 0.18 mm; the stems (20 to 7 mm) are filled at their true width.", "Hängende und überhängende Triebe (5 bis 7 mm) sind Striche von 0,18 mm; die Triebe (20 bis 7 mm) sind in wahrer Breite gefüllt."),
    hatching: T("None; laths and stems are filled strips.", "Keine; Latten und Triebe sind gefüllte Streifen."),
    styles: T("Timber mid-grey, galvanised steel light, leaf mats dark, the lower mats darkest, leaves as fine outlines.", "Holz mittelgrau, verzinkter Stahl hell, Blattteppiche dunkel, die unteren am dunkelsten, Blätter als feine Umrisse."),
    overlays: T("Mats and leaves carry their leaf regions; stems and shoots the stem region; laths, posts and rails the timber region; sleeves, plates and bolts the steel region.", "Teppiche und Blätter tragen ihre Blattregionen; Triebe die Triebregion; Latten, Pfosten und Riegel die Holzregion; Hülsen, Platten und Schrauben die Stahlregion."),
  },
  100: {
    silhouette: T(
      "At 1:100 (1 m is 10 mm) posts, rails, sleeves and base plates are filled; the lattice is a grid of fifteen filled strips of 0.2 mm and the four stems are strokes in the open strip; each leaf mat is the outline of one cushion with three-lobed leaves on it. Front and plan: the front shows the mats beside the post and the stems in front; the plan shows the mats over the rail and the base plates.",
      "Bei 1:100 (1 m sind 10 mm) sind Pfosten, Riegel, Hülsen und Fußplatten gefüllt; das Gitter ist ein Raster aus fünfzehn gefüllten Streifen von 0,2 mm und die vier Triebe sind Striche im offenen Streifen; jeder Blattteppich ist der Umriss eines Polsters mit dreilappigen Blättern darauf. Vorder- und Draufsicht: die Vorderansicht zeigt die Teppiche neben dem Pfosten und die Triebe davor; die Draufsicht zeigt die Teppiche über dem Riegel und die Fußplatten.",
    ),
    retained: T("Retained: posts, rails, sleeves, plates, all fifteen laths, the four stems, the mat outlines and leaves.", "Beibehalten: Pfosten, Riegel, Hülsen, Platten, alle fünfzehn Latten, die vier Triebe, die Teppichumrisse und Blätter."),
    merged: T("Merged: the two cushions of each mat into one; three or four leaves into one drawn leaf a fifth larger.", "Zusammengefasst: die zwei Polster jedes Teppichs zu einem; drei oder vier Blätter zu einem gezeichneten, ein Fünftel größeren Blatt."),
    omitted: T("Omitted: nuts, bolts, the head chamfer of the posts, the weave of the stems, the trailing shoots' stems.", "Weggelassen: Muttern, Schrauben, die Kopffase der Pfosten, das Flechten der Triebe, die Achsen der hängenden Triebe."),
    thinElements: T("Laths (20 mm) are filled strips of 0.2 mm; stems (20 to 7 mm) are 0.13 mm lines.", "Latten (20 mm) sind gefüllte Streifen von 0,2 mm; Triebe (20 bis 7 mm) sind Linien von 0,13 mm."),
    hatching: T("None.", "Keine."),
    styles: T("Timber mid-grey, steel light, mats dark, the lower mats darkest; leaves as fine outlines.", "Holz mittelgrau, Stahl hell, Teppiche dunkel, die unteren am dunkelsten; Blätter als feine Umrisse."),
    overlays: T("Each mat maps to its leaf region; stem strokes carry the stem region, lath strips the timber region, sleeves and plates the steel region.", "Jeder Teppich entspricht seiner Blattregion; Triebstriche tragen die Triebregion, Lattenstreifen die Holzregion, Hülsen und Platten die Stahlregion."),
  },
  200: {
    silhouette: T(
      "At 1:200 (1 m is 5 mm) the screen is the open frame of posts, rails and sleeves with eight leaf mats, four above and four below, each the ragged outline of its cushion and leaf sprays, over its upper three quarters; the strip above the bottom rail stays open and shows the four stems as 0.13 mm lines. Front and plan: the front shows the cushions beside the post; the plan shows the upper mats over the rail between the posts.",
      "Bei 1:200 (1 m sind 5 mm) ist die Wand der offene Rahmen aus Pfosten, Riegeln und Hülsen mit acht Blattteppichen, vier oben und vier unten, jeder der ausgefranste Umriss seines Polsters und seiner Blattbüschel, über ihren oberen drei Vierteln; der Streifen über dem unteren Riegel bleibt offen und zeigt die vier Triebe als Linien von 0,13 mm. Vorder- und Draufsicht: die Vorderansicht zeigt die Polster neben dem Pfosten; die Draufsicht zeigt die oberen Teppiche über dem Riegel zwischen den Pfosten.",
    ),
    retained: T("Retained: posts, rails, sleeves, the four stems and the eight cushions.", "Beibehalten: Pfosten, Riegel, Hülsen, die vier Triebe und die acht Polster."),
    merged: T("Merged: all leaves of a mat and the overhanging shoots into one cushion per bone.", "Zusammengefasst: alle Blätter eines Teppichs und die überhängenden Triebe zu einem Polster je Knochen."),
    omitted: T("Omitted: the laths (0.1 mm), base plates and single leaves.", "Weggelassen: die Latten (0,1 mm), Fußplatten und einzelne Blätter."),
    thinElements: T("The stems (20 to 7 mm) are 0.13 mm lines, a widening of up to 19 mm; the rails (70 mm) are 0.35 mm wide.", "Die Triebe (20 bis 7 mm) sind Linien von 0,13 mm, eine Verbreiterung um bis zu 19 mm; die Riegel (70 mm) sind 0,35 mm breit."),
    hatching: T("None.", "Keine."),
    styles: T("Frame mid-grey, sleeves light, cushions dark, the lower ones darkest.", "Rahmen mittelgrau, Hülsen hell, Polster dunkel, die unteren am dunkelsten."),
    overlays: T("Each cushion maps to its leaf region; posts and rails carry the timber region, sleeves the steel region.", "Jedes Polster entspricht seiner Blattregion; Pfosten und Riegel tragen die Holzregion, Hülsen die Stahlregion."),
  },
  500: {
    silhouette: T(
      "At 1:500 (1 m is 2 mm) the ivy on trellis is two posts with one leaf screen between them, 1.9 m by 1.3 m, with a ragged outline, its lower edge 0.8 m above the ground, the two rails and the four stems under the screen as lines. Front and plan: the front shows the screen 0.25 m thick on the post; the plan shows one band between the post heads.",
      "Bei 1:500 (1 m sind 2 mm) ist der Efeu am Spalier zwei Pfosten mit einer Blattwand dazwischen, 1,9 m mal 1,3 m, mit ausgefranstem Umriss, ihr unterer Rand 0,8 m über dem Boden, die zwei Riegel und die vier Triebe unter der Wand als Linien. Vorder- und Draufsicht: die Vorderansicht zeigt die 0,25 m dicke Wand am Pfosten; die Draufsicht zeigt ein Band zwischen den Pfostenköpfen.",
    ),
    retained: T("Retained: the two posts, the open strip under the screen, the two rails and the four stems.", "Beibehalten: die zwei Pfosten, der offene Streifen unter der Wand, die zwei Riegel und die vier Triebe."),
    merged: T("Merged: all leaf mats and shoots into one screen; it carries both leaf regions.", "Zusammengefasst: alle Blattteppiche und überhängenden Triebe zu einer Wand; sie trägt beide Blattregionen."),
    omitted: T("Omitted: sleeves and plates (0.2 mm), laths and leaves; the posts carry the steel region with the timber region.", "Weggelassen: Hülsen und Platten (0,2 mm), Latten und Blätter; die Pfosten tragen die Stahlregion mit der Holzregion."),
    thinElements: T("The rails (70 mm = 0.14 mm) and the posts (90 mm) are filled at their true width; the stems (20 to 7 mm) are lines of 0.1 mm, a widening to 50 mm.", "Die Riegel (70 mm = 0,14 mm) und die Pfosten (90 mm) sind in wahrer Breite gefüllt; die Triebe (20 bis 7 mm) sind Linien von 0,1 mm, eine Verbreiterung auf 50 mm."),
    hatching: T("None.", "Keine."),
    styles: T("Posts mid-grey, the screen dark; readable at 4 mm printed height.", "Pfosten mittelgrau, die Wand dunkel; bei 4 mm Druckhöhe lesbar."),
    overlays: T("The screen shows the area-weighted value of the two leaf regions, labelled as merged; the posts that of timber and steel.", "Die Wand zeigt den flächengewichteten Wert der zwei Blattregionen, als zusammengefasst beschriftet; die Pfosten den von Holz und Stahl."),
  },
};
