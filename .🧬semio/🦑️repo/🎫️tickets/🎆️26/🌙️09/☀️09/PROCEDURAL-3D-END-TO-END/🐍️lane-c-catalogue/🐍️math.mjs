import { T, I, O, P, K, pick, gumball } from "./🐍️lib.mjs";

const Q = "exact-analytic";
const PI = Math.PI;

const num = (name, en, de, den, dde, def = 0, extra = {}) => P(name, "number", T(en, de), T(den, dde), { default: def, ...extra });
const numOut = (name, en, de, den, dde) => O(name, "number", T(en, de), T(den, dde));
const pointIn = (name, en, de, den, dde, def = [0, 0, 0]) => P(name, "point", T(en, de), T(den, dde), { default: def });
const vecIn = (name, en, de, den, dde, def = [0, 0, 1]) => P(name, "vector", T(en, de), T(den, dde), { default: def });
const result = (den, dde) => numOut("result", "Result", "Ergebnis", den, dde);

const RES = {
  "math.add": ["The sum of A and B.", "Die Summe aus A und B."],
  "math.subtract": ["The difference A minus B.", "Die Differenz A minus B."],
  "math.multiply": ["The product of A and B.", "Das Produkt aus A und B."],
  "math.divide": ["The quotient A divided by B.", "Der Quotient A durch B."],
  "math.modulo": ["The remainder of A divided by B.", "Der Rest von A geteilt durch B."],
  "math.minimum": ["The smaller of A and B.", "Die kleinere der Zahlen A und B."],
  "math.maximum": ["The larger of A and B.", "Die größere der Zahlen A und B."],
};

const binary = (id, was, emoji, en, de, den, dde, defA = 0, defB = 0) =>
  K("math.arithmetic", {
    id, emoji, quality: Q, was,
    label: T(en, de), description: T(den, dde),
    in: [num("a", "A", "A", "The first number.", "Die erste Zahl.", defA), num("b", "B", "B", "The second number.", "Die zweite Zahl.", defB)],
    out: [result(...RES[id])],
  });

export const values = {
  category: {
    id: "math.values", emoji: "🔢", order: 400,
    label: T("Values", "Werte"),
    description: T("Constants that feed numbers, angles, vectors, points and planes into other widgets.", "Konstanten, die Zahlen, Winkel, Vektoren, Punkte und Ebenen in andere Widgets einspeisen."),
  },
  kinds: [
    K("math.values", { id: "math.number", emoji: "🔢", quality: Q, label: T("Number", "Zahl"), description: T("Provides a decimal number.", "Liefert eine Dezimalzahl."), in: [num("value", "Value", "Wert", "The number.", "Die Zahl.")], out: [numOut("value", "Value", "Wert", "The number.", "Die Zahl.")] }),
    K("math.values", { id: "math.integer", emoji: "🔟", quality: Q, label: T("Integer", "Ganzzahl"), description: T("Provides a whole number.", "Liefert eine ganze Zahl."), in: [P("value", "integer", T("Value", "Wert"), T("The whole number.", "Die ganze Zahl."), { default: 0 })], out: [O("value", "integer", T("Value", "Wert"), T("The whole number.", "Die ganze Zahl."))] }),
    K("math.values", { id: "math.angle", emoji: "📐", quality: Q, label: T("Angle", "Winkel"), description: T("Provides an angle, edited in degrees and stored in radians.", "Liefert einen Winkel, in Grad bearbeitet und in Bogenmaß gespeichert."), in: [I("angle", { as: "value", default: 0, label: T("Value", "Wert"), description: T("The angle.", "Der Winkel.") })], out: [O("value", "angle", T("Value", "Wert"), T("The angle in radians.", "Der Winkel im Bogenmaß."))] }),
    K("math.values", { id: "math.length", emoji: "📏", quality: Q, label: T("Length", "Länge"), description: T("Provides a length in model units.", "Liefert eine Länge in Modelleinheiten."), in: [P("value", "length", T("Value", "Wert"), T("The length.", "Die Länge."), { default: 1 })], out: [O("value", "length", T("Value", "Wert"), T("The length.", "Die Länge."))] }),
    K("math.values", { id: "math.boolean", emoji: "☑", quality: Q, label: T("Boolean", "Wahrheitswert"), description: T("Provides a yes or no value.", "Liefert einen Ja-Nein-Wert."), in: [P("value", "boolean", T("Value", "Wert"), T("True or false.", "Wahr oder falsch."), { default: false })], out: [O("value", "boolean", T("Value", "Wert"), T("True or false.", "Wahr oder falsch."))] }),
    K("math.values", {
      id: "math.vector", emoji: "➡", quality: Q,
      label: T("Vector", "Vektor"), description: T("Provides a direction or displacement with three components.", "Liefert eine Richtung oder Verschiebung mit drei Komponenten."),
      in: [vecIn("value", "Value", "Wert", "The vector.", "Der Vektor.", [0, 0, 1])],
      out: [O("value", "vector", T("Value", "Wert"), T("The vector.", "Der Vektor."))],
      interaction: { gumball: [gumball("translate", "value")] },
    }),
    K("math.values", {
      id: "math.point", emoji: "📍", quality: Q,
      label: T("Point", "Punkt"), description: T("Provides a position in space.", "Liefert eine Position im Raum."),
      in: [pointIn("value", "Value", "Wert", "The position.", "Die Position.")],
      out: [O("value", "point", T("Value", "Wert"), T("The position.", "Die Position."))],
      interaction: { gumball: [gumball("translate", "value")] },
    }),
    K("math.values", {
      id: "math.plane", emoji: "🔳", quality: Q,
      label: T("Plane", "Ebene"), description: T("Provides a plane from an origin and a normal.", "Liefert eine Ebene aus Ursprung und Normale."),
      in: [I("plane", { as: "value", label: T("Value", "Wert"), description: T("The plane.", "Die Ebene.") })],
      out: [O("value", "plane", T("Value", "Wert"), T("The plane.", "Die Ebene."))],
      interaction: { gumball: [gumball("translate", "value"), gumball("rotate", "value")] },
    }),
  ],
};

export const arithmetic = {
  category: {
    id: "math.arithmetic", emoji: "🧮", order: 410,
    label: T("Arithmetic", "Arithmetik"),
    description: T("Calculate with numbers: the four basic operations, powers, rounding, limits and trigonometry.", "Mit Zahlen rechnen: Grundrechenarten, Potenzen, Runden, Grenzen und Trigonometrie."),
  },
  kinds: [
    binary("math.add", undefined, "➕", "Add", "Addieren", "Adds two numbers.", "Addiert zwei Zahlen."),
    binary("math.subtract", undefined, "➖", "Subtract", "Subtrahieren", "Subtracts B from A.", "Subtrahiert B von A."),
    binary("math.multiply", undefined, "✖", "Multiply", "Multiplizieren", "Multiplies two numbers.", "Multipliziert zwei Zahlen.", 1, 1),
    binary("math.divide", undefined, "➗", "Divide", "Dividieren", "Divides A by B; B must not be zero.", "Dividiert A durch B; B darf nicht null sein.", 1, 1),
    binary("math.modulo", undefined, "🍰", "Remainder", "Rest", "Gives the remainder of dividing A by B; B must not be zero.", "Liefert den Rest der Division von A durch B; B darf nicht null sein.", 1, 2),
    K("math.arithmetic", { id: "math.power", emoji: "⚡", quality: Q, label: T("Power", "Potenz"), description: T("Raises a base to an exponent.", "Potenziert eine Basis mit einem Exponenten."), in: [num("base", "Base", "Basis", "The number to raise.", "Die zu potenzierende Zahl.", 2), num("exponent", "Exponent", "Exponent", "The power to raise it to.", "Die Hochzahl.", 2)], out: [result("The base raised to the exponent.", "Die Basis hoch Exponent.")] }),
    K("math.arithmetic", { id: "math.negate", emoji: "🔀", quality: Q, label: T("Negate", "Negieren"), description: T("Flips the sign of a number.", "Kehrt das Vorzeichen einer Zahl um."), in: [num("value", "Value", "Wert", "The number.", "Die Zahl.")], out: [result("The number with the opposite sign.", "Die Zahl mit umgekehrtem Vorzeichen.")] }),
    K("math.arithmetic", { id: "math.absolute", emoji: "🧲", quality: Q, label: T("Absolute value", "Betrag"), description: T("Removes the sign of a number.", "Entfernt das Vorzeichen einer Zahl."), in: [num("value", "Value", "Wert", "The number.", "Die Zahl.", -1)], out: [result("The distance of the number from zero.", "Der Abstand der Zahl von null.")] }),
    K("math.arithmetic", { id: "math.squareRoot", emoji: "🌱", quality: Q, label: T("Square root", "Quadratwurzel"), description: T("Takes the square root of a non-negative number.", "Zieht die Quadratwurzel einer nichtnegativen Zahl."), in: [num("value", "Value", "Wert", "The number; it must not be negative.", "Die Zahl; sie darf nicht negativ sein.", 4, { min: 0 })], out: [result("The square root.", "Die Quadratwurzel.")] }),
    K("math.arithmetic", {
      id: "math.round", emoji: "🎯", quality: Q, label: T("Round", "Runden"), description: T("Rounds a number to a whole number.", "Rundet eine Zahl auf eine ganze Zahl."),
      in: [num("value", "Value", "Wert", "The number to round.", "Die zu rundende Zahl.", 0.5), P("mode", "enum", T("Direction", "Richtung"), T("How the number is rounded.", "Wie die Zahl gerundet wird."), { default: "nearest", options: [{ value: "nearest", label: T("To nearest", "Zur nächsten") }, { value: "down", label: T("Down", "Abrunden") }, { value: "up", label: T("Up", "Aufrunden") }] })],
      out: [O("result", "integer", T("Result", "Ergebnis"), T("The rounded whole number.", "Die gerundete ganze Zahl."))],
    }),
    binary("math.minimum", undefined, "⬇", "Minimum", "Minimum", "Picks the smaller of two numbers.", "Wählt die kleinere von zwei Zahlen."),
    binary("math.maximum", undefined, "⬆", "Maximum", "Maximum", "Picks the larger of two numbers.", "Wählt die größere von zwei Zahlen."),
    K("math.arithmetic", { id: "math.clamp", emoji: "🗜", quality: Q, label: T("Clamp", "Begrenzen"), description: T("Limits a number to a range.", "Begrenzt eine Zahl auf einen Bereich."), in: [num("value", "Value", "Wert", "The number to limit.", "Die zu begrenzende Zahl.", 0.5), num("lower", "Lower limit", "Untergrenze", "The smallest allowed value.", "Der kleinste erlaubte Wert.", 0), num("upper", "Upper limit", "Obergrenze", "The largest allowed value.", "Der größte erlaubte Wert.", 1)], out: [result("The number limited to the range.", "Die auf den Bereich begrenzte Zahl.")] }),
    K("math.arithmetic", { id: "math.interpolate", emoji: "🎚", quality: Q, label: T("Interpolate", "Interpolieren"), description: T("Blends between two numbers.", "Überblendet zwischen zwei Zahlen."), in: [num("a", "From", "Von", "The value at 0.", "Der Wert bei 0.", 0), num("b", "To", "Nach", "The value at 1.", "Der Wert bei 1.", 1), num("t", "Blend", "Anteil", "Blend factor; 0 gives From and 1 gives To.", "Mischfaktor; 0 ergibt Von und 1 ergibt Nach.", 0.5, { step: 0.05 })], out: [result("The blended number.", "Die überblendete Zahl.")] }),
    K("math.arithmetic", { id: "math.sine", emoji: "🌊", quality: Q, label: T("Sine", "Sinus"), description: T("Takes the sine of an angle.", "Bildet den Sinus eines Winkels."), in: [I("angle", { default: 0, label: T("Angle", "Winkel"), description: T("The angle.", "Der Winkel.") })], out: [result("The sine, between -1 and 1.", "Der Sinus, zwischen -1 und 1.")] }),
    K("math.arithmetic", { id: "math.cosine", emoji: "〰", quality: Q, label: T("Cosine", "Kosinus"), description: T("Takes the cosine of an angle.", "Bildet den Kosinus eines Winkels."), in: [I("angle", { default: 0, label: T("Angle", "Winkel"), description: T("The angle.", "Der Winkel.") })], out: [result("The cosine, between -1 and 1.", "Der Kosinus, zwischen -1 und 1.")] }),
    K("math.arithmetic", { id: "math.tangent", emoji: "📈", quality: Q, label: T("Tangent", "Tangens"), description: T("Takes the tangent of an angle; it is undefined at 90 degrees.", "Bildet den Tangens eines Winkels; bei 90 Grad ist er nicht definiert."), in: [I("angle", { default: 0, label: T("Angle", "Winkel"), description: T("The angle.", "Der Winkel.") })], out: [result("The tangent.", "Der Tangens.")] }),
  ],
};

const xyz = (en, de, defs = [0, 0, 0]) => [
  num("x", "X", "X", `The X component of the ${en}.`, `Die X-Komponente ${de}.`, defs[0]),
  num("y", "Y", "Y", `The Y component of the ${en}.`, `Die Y-Komponente ${de}.`, defs[1]),
  num("z", "Z", "Z", `The Z component of the ${en}.`, `Die Z-Komponente ${de}.`, defs[2]),
];
const xyzOut = (en, de) => [
  numOut("x", "X", "X", `The X component of the ${en}.`, `Die X-Komponente ${de}.`),
  numOut("y", "Y", "Y", `The Y component of the ${en}.`, `Die Y-Komponente ${de}.`),
  numOut("z", "Z", "Z", `The Z component of the ${en}.`, `Die Z-Komponente ${de}.`),
];
const vec2 = (den, dde) => [vecIn("a", "Vector A", "Vektor A", "The first vector.", "Der erste Vektor.", [1, 0, 0]), vecIn("b", "Vector B", "Vektor B", "The second vector.", "Der zweite Vektor.", [0, 1, 0])];

export const vector = {
  category: {
    id: "math.vector", emoji: "➡", order: 420,
    label: T("Vectors, points and planes", "Vektoren, Punkte und Ebenen"),
    description: T("Build, take apart and combine vectors, points and planes.", "Vektoren, Punkte und Ebenen aufbauen, zerlegen und kombinieren."),
  },
  kinds: [
    K("math.vector", { id: "math.vectorFromComponents", emoji: "🧱", quality: Q, label: T("Vector from components", "Vektor aus Komponenten"), description: T("Builds a vector from three numbers.", "Baut einen Vektor aus drei Zahlen."), in: xyz("vector", "des Vektors", [0, 0, 1]), out: [O("vector", "vector", T("Vector", "Vektor"), T("The vector.", "Der Vektor."))] }),
    K("math.vector", { id: "math.pointFromComponents", emoji: "📍", quality: Q, label: T("Point from coordinates", "Punkt aus Koordinaten"), description: T("Builds a point from three coordinates.", "Baut einen Punkt aus drei Koordinaten."), in: xyz("point", "des Punkts"), out: [O("point", "point", T("Point", "Punkt"), T("The point.", "Der Punkt."))] }),
    K("math.vector", { id: "math.vectorComponents", emoji: "🧩", quality: Q, label: T("Vector components", "Vektorkomponenten"), description: T("Splits a vector into its three components.", "Zerlegt einen Vektor in seine drei Komponenten."), in: [vecIn("vector", "Vector", "Vektor", "The vector to split.", "Der zu zerlegende Vektor.")], out: xyzOut("vector", "des Vektors") }),
    K("math.vector", { id: "math.pointComponents", emoji: "📌", quality: Q, label: T("Point coordinates", "Punktkoordinaten"), description: T("Splits a point into its three coordinates.", "Zerlegt einen Punkt in seine drei Koordinaten."), in: [pointIn("point", "Point", "Punkt", "The point to split.", "Der zu zerlegende Punkt.")], out: xyzOut("point", "des Punkts") }),
    K("math.vector", { id: "math.vectorAdd", emoji: "➕", quality: Q, label: T("Add vectors", "Vektoren addieren"), description: T("Adds two vectors.", "Addiert zwei Vektoren."), in: vec2(), out: [O("vector", "vector", T("Sum", "Summe"), T("The sum of A and B.", "Die Summe aus A und B."))] }),
    K("math.vector", { id: "math.vectorSubtract", emoji: "➖", quality: Q, label: T("Subtract vectors", "Vektoren subtrahieren"), description: T("Subtracts vector B from vector A.", "Subtrahiert Vektor B von Vektor A."), in: vec2(), out: [O("vector", "vector", T("Difference", "Differenz"), T("The difference A minus B.", "Die Differenz A minus B."))] }),
    K("math.vector", { id: "math.vectorScale", emoji: "✖", quality: Q, label: T("Scale vector", "Vektor skalieren"), description: T("Multiplies a vector by a number.", "Multipliziert einen Vektor mit einer Zahl."), in: [vecIn("vector", "Vector", "Vektor", "The vector to scale.", "Der zu skalierende Vektor.", [1, 0, 0]), num("factor", "Factor", "Faktor", "The multiplier.", "Der Multiplikator.", 2)], out: [O("vector", "vector", T("Scaled vector", "Skalierter Vektor"), T("The scaled vector.", "Der skalierte Vektor."))] }),
    K("math.vector", { id: "math.vectorLength", emoji: "📏", quality: Q, label: T("Vector length", "Vektorlänge"), description: T("Measures the length of a vector.", "Misst die Länge eines Vektors."), in: [vecIn("vector", "Vector", "Vektor", "The vector to measure.", "Der zu messende Vektor.", [1, 0, 0])], out: [O("length", "length", T("Length", "Länge"), T("The length of the vector.", "Die Länge des Vektors."))] }),
    K("math.vector", { id: "math.vectorNormalize", emoji: "🎛", quality: Q, label: T("Normalize vector", "Vektor normieren"), description: T("Scales a vector to length one; the zero vector has no direction.", "Skaliert einen Vektor auf Länge eins; der Nullvektor hat keine Richtung."), in: [vecIn("vector", "Vector", "Vektor", "The vector to normalize; it must not be zero.", "Der zu normierende Vektor; er darf nicht null sein.", [2, 0, 0])], out: [O("vector", "vector", T("Unit vector", "Einheitsvektor"), T("The vector with length one.", "Der Vektor mit Länge eins."))] }),
    K("math.vector", { id: "math.vectorDot", emoji: "⚫", quality: Q, label: T("Dot product", "Skalarprodukt"), description: T("Computes the dot product of two vectors.", "Berechnet das Skalarprodukt zweier Vektoren."), in: vec2(), out: [numOut("result", "Dot product", "Skalarprodukt", "Sum of the component products; zero when the vectors are perpendicular.", "Summe der Komponentenprodukte; null bei senkrechten Vektoren.")] }),
    K("math.vector", { id: "math.vectorCross", emoji: "❌", quality: Q, label: T("Cross product", "Kreuzprodukt"), description: T("Computes the vector perpendicular to two vectors.", "Berechnet den Vektor senkrecht zu zwei Vektoren."), in: vec2(), out: [O("vector", "vector", T("Cross product", "Kreuzprodukt"), T("The perpendicular vector following the right-hand rule.", "Der senkrechte Vektor nach der Rechte-Hand-Regel."))] }),
    K("math.vector", { id: "math.vectorAngle", emoji: "📐", quality: Q, label: T("Angle between vectors", "Winkel zwischen Vektoren"), description: T("Measures the angle between two vectors.", "Misst den Winkel zwischen zwei Vektoren."), in: vec2(), out: [O("angle", "angle", T("Angle", "Winkel"), T("The angle from 0 to 180 degrees.", "Der Winkel von 0 bis 180 Grad."))] }),
    K("math.vector", { id: "math.pointOffset", emoji: "🛫", quality: Q, label: T("Offset point", "Punkt versetzen"), description: T("Moves a point by a vector.", "Verschiebt einen Punkt um einen Vektor."), in: [pointIn("point", "Point", "Punkt", "The starting position.", "Die Ausgangsposition."), vecIn("offset", "Offset", "Versatz", "The displacement.", "Die Verschiebung.", [0, 0, 1])], out: [O("point", "point", T("Point", "Punkt"), T("The moved point.", "Der verschobene Punkt."))] }),
    K("math.vector", { id: "math.pointDistance", emoji: "↔", quality: Q, label: T("Distance between points", "Abstand zwischen Punkten"), description: T("Measures the straight distance between two points.", "Misst den geraden Abstand zwischen zwei Punkten."), in: [pointIn("a", "Point A", "Punkt A", "The first point.", "Der erste Punkt."), pointIn("b", "Point B", "Punkt B", "The second point.", "Der zweite Punkt.", [1, 0, 0])], out: [O("distance", "length", T("Distance", "Abstand"), T("The distance between the points.", "Der Abstand zwischen den Punkten."))] }),
    K("math.vector", { id: "math.pointInterpolate", emoji: "🎚", quality: Q, label: T("Point between", "Punkt dazwischen"), description: T("Finds the point at a fraction of the way between two points.", "Findet den Punkt bei einem Bruchteil des Wegs zwischen zwei Punkten."), in: [pointIn("a", "From", "Von", "The point at 0.", "Der Punkt bei 0."), pointIn("b", "To", "Nach", "The point at 1.", "Der Punkt bei 1.", [1, 0, 0]), num("t", "Fraction", "Anteil", "Position between the points; 0.5 is the midpoint.", "Position zwischen den Punkten; 0,5 ist die Mitte.", 0.5, { step: 0.05 })], out: [O("point", "point", T("Point", "Punkt"), T("The interpolated point.", "Der interpolierte Punkt."))] }),
    K("math.vector", { id: "math.vectorBetween", emoji: "🏹", quality: Q, label: T("Vector between points", "Vektor zwischen Punkten"), description: T("Builds the vector that leads from one point to another.", "Baut den Vektor, der von einem Punkt zu einem anderen führt."), in: [pointIn("from", "From", "Von", "The starting point.", "Der Startpunkt."), pointIn("to", "To", "Nach", "The target point.", "Der Zielpunkt.", [1, 0, 0])], out: [O("vector", "vector", T("Vector", "Vektor"), T("The vector from the first to the second point.", "Der Vektor vom ersten zum zweiten Punkt."))] }),
    K("math.vector", { id: "math.planeFromPointNormal", emoji: "🔳", quality: Q, label: T("Plane from point and normal", "Ebene aus Punkt und Normale"), description: T("Builds a plane through a point with a given normal.", "Baut eine Ebene durch einen Punkt mit gegebener Normale."), in: [pointIn("origin", "Origin", "Ursprung", "A point on the plane.", "Ein Punkt auf der Ebene."), vecIn("normal", "Normal", "Normale", "Direction perpendicular to the plane.", "Richtung senkrecht zur Ebene.")], out: [O("plane", "plane", T("Plane", "Ebene"), T("The plane.", "Die Ebene."))] }),
    K("math.vector", { id: "math.planeFromPoints", emoji: "🔺", quality: Q, label: T("Plane from three points", "Ebene aus drei Punkten"), description: T("Builds the plane through three points that do not lie on a line.", "Baut die Ebene durch drei Punkte, die nicht auf einer Geraden liegen."), in: [pointIn("a", "Point A", "Punkt A", "The first point; it becomes the origin.", "Der erste Punkt; er wird zum Ursprung."), pointIn("b", "Point B", "Punkt B", "The second point.", "Der zweite Punkt.", [1, 0, 0]), pointIn("c", "Point C", "Punkt C", "The third point.", "Der dritte Punkt.", [0, 1, 0])], out: [O("plane", "plane", T("Plane", "Ebene"), T("The plane through the points.", "Die Ebene durch die Punkte."))] }),
    K("math.vector", { id: "math.planeComponents", emoji: "🪓", quality: Q, label: T("Plane components", "Ebenenkomponenten"), description: T("Splits a plane into its origin and normal.", "Zerlegt eine Ebene in Ursprung und Normale."), in: [I("plane", { description: T("The plane to split.", "Die zu zerlegende Ebene.") })], out: [O("origin", "point", T("Origin", "Ursprung"), T("A point on the plane.", "Ein Punkt auf der Ebene.")), O("normal", "vector", T("Normal", "Normale"), T("The unit normal of the plane.", "Die Einheitsnormale der Ebene."))] }),
  ],
};

export const list = {
  category: {
    id: "math.list", emoji: "📚", order: 430,
    label: T("Ranges and lists", "Bereiche und Listen"),
    description: T("Create number sequences and read items from lists.", "Zahlenfolgen erzeugen und Elemente aus Listen lesen."),
  },
  kinds: [
    K("math.list", {
      id: "math.range", emoji: "📏", quality: Q, label: T("Range", "Bereich"), description: T("Creates evenly spaced numbers from a start to an end value, both included.", "Erzeugt gleichmäßig verteilte Zahlen von einem Start- bis zu einem Endwert, beide eingeschlossen."),
      in: [num("start", "Start", "Start", "The first number.", "Die erste Zahl.", 0), num("end", "End", "Ende", "The last number.", "Die letzte Zahl.", 1), P("count", "integer", T("Count", "Anzahl"), T("How many numbers to create; at least two.", "Wie viele Zahlen erzeugt werden; mindestens zwei."), { default: 11, min: 2, max: 100000 })],
      out: [O("numbers", "number", T("Numbers", "Zahlen"), T("The evenly spaced numbers.", "Die gleichmäßig verteilten Zahlen."), { list: true })],
    }),
    K("math.list", {
      id: "math.series", emoji: "🪜", quality: Q, label: T("Series", "Folge"), description: T("Creates numbers that grow by a fixed step.", "Erzeugt Zahlen, die um eine feste Schrittweite wachsen."),
      in: [num("start", "Start", "Start", "The first number.", "Die erste Zahl.", 0), num("step", "Step", "Schritt", "The difference between neighbouring numbers.", "Die Differenz zwischen benachbarten Zahlen.", 1), P("count", "integer", T("Count", "Anzahl"), T("How many numbers to create.", "Wie viele Zahlen erzeugt werden."), { default: 10, min: 1, max: 100000 })],
      out: [O("numbers", "number", T("Numbers", "Zahlen"), T("The numbers of the series.", "Die Zahlen der Folge."), { list: true })],
    }),
    K("math.list", {
      id: "math.listItem", emoji: "🎣", quality: Q, label: T("List item", "Listenelement"), description: T("Reads one item from a list by its zero-based position.", "Liest ein Element einer Liste anhand seiner nullbasierten Position."),
      in: [P("list", "any", T("List", "Liste"), T("The list to read from.", "Die Liste, aus der gelesen wird."), { list: true, default: [] }), P("index", "integer", T("Index", "Position"), T("Zero-based position of the item.", "Nullbasierte Position des Elements."), { default: 0, min: 0 })],
      out: [O("item", "any", T("Item", "Element"), T("The item at the position.", "Das Element an der Position."))],
    }),
    K("math.list", {
      id: "math.listLength", emoji: "🔢", quality: Q, label: T("List length", "Listenlänge"), description: T("Counts the items in a list.", "Zählt die Elemente einer Liste."),
      in: [P("list", "any", T("List", "Liste"), T("The list to count.", "Die zu zählende Liste."), { list: true, default: [] })],
      out: [O("count", "integer", T("Count", "Anzahl"), T("The number of items.", "Die Anzahl der Elemente."))],
    }),
  ],
};
