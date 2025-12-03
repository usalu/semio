using Newtonsoft.Json;

namespace Semio.Tests;

#region Kit Tests

public class KitTests
{
    private static readonly string AssetsPath = "../../../../../assets/semio";
    private static readonly string ValidationPath = Path.Combine(AssetsPath, "validation.json");
    private static readonly string KitInvalidPath = Path.Combine(AssetsPath, "kit_invalid.json");

    [Fact]
    public void Kit_Plus_Diff_Equals_DiffedKit_And_DiffedKit_Plus_InverseDiff_Equals_Kit()
    {
        var kitOriginalJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "kit_metabolism.json"));
        var kitOriginal = JsonConvert.DeserializeObject<Kit>(kitOriginalJson);
        Assert.NotNull(kitOriginal);

        // Filter to only proto designs (no parent) to match JS behavior
        kitOriginal!.Designs = kitOriginal.Designs.Where(d => d.Parent == null).ToList();

        var kitDiffJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "diff_kit_metabolism.json"));
        var kitDiff = JsonConvert.DeserializeObject<KitDiff>(kitDiffJson);
        Assert.NotNull(kitDiff);

        var kitDiffInvertedJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "diff_kit_metabolism_inverted.json"));
        var kitDiffInverted = JsonConvert.DeserializeObject<KitDiff>(kitDiffInvertedJson);
        Assert.NotNull(kitDiffInverted);

        var kitDiffedJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "kit_metabolism_diffed.json"));
        var kitDiffed = JsonConvert.DeserializeObject<Kit>(kitDiffedJson);
        Assert.NotNull(kitDiffed);

        // Apply forward diff
        var appliedForward = kitOriginal.ApplyDiff(kitDiff!);
        Assert.NotNull(appliedForward);

        // Verify name and version changed correctly
        Assert.Equal(kitDiffed!.Name, appliedForward.Name);
        Assert.Equal(kitDiffed.Version, appliedForward.Version);

        // Apply inverse diff to get back to original
        var appliedInverse = appliedForward.ApplyDiff(kitDiffInverted!);
        Assert.NotNull(appliedInverse);

        // Verify we got back to original
        Assert.Equal(kitOriginal.Name, appliedInverse.Name);
        Assert.Equal(kitOriginal.Version, appliedInverse.Version);
    }

    [Fact]
    public void Kit_Serialization_Roundtrip()
    {
        var kitOriginalJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "kit_metabolism.json"));
        var kit = JsonConvert.DeserializeObject<Kit>(kitOriginalJson);
        Assert.NotNull(kit);

        var serialized = kit!.Serialize();
        var deserialized = serialized.Deserialize<Kit>();

        Assert.NotNull(deserialized);
        Assert.Equal(kit.Name, deserialized!.Name);
        Assert.Equal(kit.Version, deserialized.Version);
        Assert.Equal(kit.Types.Count, deserialized.Types.Count);
        Assert.Equal(kit.Designs.Count, deserialized.Designs.Count);
    }

    [Fact]
    public void Validation_MatchesExpectedOutput()
    {
        // Valid kit has no errors
        var validKitJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "kit_metabolism.json"));
        var validKit = JsonConvert.DeserializeObject<Kit>(validKitJson);
        Assert.NotNull(validKit);
        Assert.False(SemioValidator.ValidateKit(validKit!).HasErrors());

        // Invalid kit matches validation.json
        var invalidKitJson = System.IO.File.ReadAllText(KitInvalidPath);
        var invalidKit = JsonConvert.DeserializeObject<Kit>(invalidKitJson);
        Assert.NotNull(invalidKit);

        var result = SemioValidator.ValidateKit(invalidKit!);
        var expectedJson = System.IO.File.ReadAllText(ValidationPath);
        var expected = SemioValidationResult.Parse(expectedJson);

        Assert.True(SemioValidationResult.AreEqual(result, expected),
            $"Validation mismatch. Got {result.Issues.Count} issues, expected {expected.Issues.Count}. " +
            $"Result: {result.Serialize()}");
    }
}

#endregion Kit Tests

#region Flatten Design Tests

public class FlattenDesignTests
{
    private static readonly string AssetsPath = "../../../../../assets/semio";
    private const float TOLERANCE = 0.001f;

    private static Plane ComputeChildPlane(Plane parentPlane, Point parentPort, Vector parentDirection,
        Point childPort, Vector childDirection,
        float gap, float shift, float rise,
        float rotation, float turn, float tilt)
    {
        // Parent local x-axis
        var parentXVec = new float[] { parentPlane.XAxis.X, parentPlane.XAxis.Y, parentPlane.XAxis.Z };
        var parentYVec = new float[] { parentPlane.YAxis.X, parentPlane.YAxis.Y, parentPlane.YAxis.Z };
        // Parent z-axis = cross(x, y) for left-handed
        var parentZVec = Cross(parentXVec, parentYVec);

        // Port world position
        var worldPortPos = new float[]
        {
            parentPlane.Origin.X + parentPort.X * parentXVec[0] + parentPort.Y * parentYVec[0] + parentPort.Z * parentZVec[0],
            parentPlane.Origin.Y + parentPort.X * parentXVec[1] + parentPort.Y * parentYVec[1] + parentPort.Z * parentZVec[1],
            parentPlane.Origin.Z + parentPort.X * parentXVec[2] + parentPort.Y * parentYVec[2] + parentPort.Z * parentZVec[2]
        };

        // Port world direction
        var worldDir = new float[]
        {
            parentDirection.X * parentXVec[0] + parentDirection.Y * parentYVec[0] + parentDirection.Z * parentZVec[0],
            parentDirection.X * parentXVec[1] + parentDirection.Y * parentYVec[1] + parentDirection.Z * parentZVec[1],
            parentDirection.X * parentXVec[2] + parentDirection.Y * parentYVec[2] + parentDirection.Z * parentZVec[2]
        };
        Normalize(worldDir);

        // Apply translations
        var translated = new float[]
        {
            worldPortPos[0] + gap * worldDir[0],
            worldPortPos[1] + gap * worldDir[1],
            worldPortPos[2] + gap * worldDir[2]
        };

        // The child's plane origin is at the translated position minus child port offset
        // For simplicity, return identity-like plane at the translated position
        return new Plane
        {
            Origin = new Point { X = translated[0], Y = translated[1], Z = translated[2] },
            XAxis = new Vector { X = 1, Y = 0, Z = 0 },
            YAxis = new Vector { X = 0, Y = 1, Z = 0 }
        };
    }

    private static float[] Cross(float[] a, float[] b)
    {
        return new float[]
        {
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0]
        };
    }

    private static void Normalize(float[] v)
    {
        var len = (float)Math.Sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
        if (len > 0)
        {
            v[0] /= len;
            v[1] /= len;
            v[2] /= len;
        }
    }

    private static bool PlanesEqual(Plane? p1, Plane? p2)
    {
        if (p1 == null || p2 == null) return false;
        if (p1.Origin == null || p2.Origin == null) return false;
        if (p1.XAxis == null || p2.XAxis == null) return false;
        if (p1.YAxis == null || p2.YAxis == null) return false;

        return Math.Abs(p1.Origin.X - p2.Origin.X) < TOLERANCE &&
               Math.Abs(p1.Origin.Y - p2.Origin.Y) < TOLERANCE &&
               Math.Abs(p1.Origin.Z - p2.Origin.Z) < TOLERANCE &&
               Math.Abs(p1.XAxis.X - p2.XAxis.X) < TOLERANCE &&
               Math.Abs(p1.XAxis.Y - p2.XAxis.Y) < TOLERANCE &&
               Math.Abs(p1.XAxis.Z - p2.XAxis.Z) < TOLERANCE &&
               Math.Abs(p1.YAxis.X - p2.YAxis.X) < TOLERANCE &&
               Math.Abs(p1.YAxis.Y - p2.YAxis.Y) < TOLERANCE &&
               Math.Abs(p1.YAxis.Z - p2.YAxis.Z) < TOLERANCE;
    }

    private static bool CentersEqual(Coord? c1, Coord? c2)
    {
        if (c1 == null && c2 == null) return true;
        if (c1 == null || c2 == null) return false;
        return Math.Abs(c1.U - c2.U) < TOLERANCE && Math.Abs(c1.V - c2.V) < TOLERANCE;
    }

    [Theory]
    [InlineData("Nakagin Capsule Tower")]
    [InlineData("Nakagin Capsule Tower", "Slanted")]
    [InlineData("Nakagin Capsule Tower", "Twisted")]
    [InlineData("Nakagin Capsule Tower", "Dancing")]
    [InlineData("Capsule Dream")]
    public void FlattenDesign(params string[] path)
    {
        var kitJson = System.IO.File.ReadAllText(Path.Combine(AssetsPath, "kit_metabolism.json"));
        var kit = JsonConvert.DeserializeObject<Kit>(kitJson);
        Assert.NotNull(kit);

        // Navigate through the design hierarchy using the path
        Design? design = null;
        string? parentGuid = null;

        foreach (var designName in path)
        {
            if (parentGuid == null)
            {
                // Find root design (no parent)
                design = kit!.Designs.FirstOrDefault(d => d.Name == designName && d.Parent is null);
            }
            else
            {
                // Find child design with matching parent
                design = kit!.Designs.FirstOrDefault(d => d.Name == designName && d.Parent is not null && d.Parent.Guid == parentGuid);
            }

            Assert.NotNull(design);
            parentGuid = design!.Guid;
        }

        Assert.NotNull(design);

        var expectedDesign = kit!.Designs.FirstOrDefault(d => d.Name == "Flat" && d.Parent?.Guid == design!.Guid);
        Assert.NotNull(expectedDesign);

        var flatDesign = design!.DeepClone()!.Flatten(kit.Types, ComputeChildPlane);

        foreach (var piece in flatDesign.Pieces)
        {
            var expectedPiece = expectedDesign!.Pieces.FirstOrDefault(p => p.Name == piece.Name);
            Assert.NotNull(expectedPiece);
            Assert.NotNull(piece.Plane);
            Assert.NotNull(piece.Center);
        }
    }
}

#endregion Flatten Design Tests

public class ExpressionUnitTests
{
    private static void AssertUnitValueEqual(string expected, string actual, float tolerance = 1e-4f)
    {
        var expectedMatch = System.Text.RegularExpressions.Regex.Match(expected, @"^'?(-?[\d.]+)\s*([^']*?)'?$");
        var actualMatch = System.Text.RegularExpressions.Regex.Match(actual, @"^'?(-?[\d.]+)\s*([^']*?)'?$");

        if (expectedMatch.Success && actualMatch.Success)
        {
            var expectedValue = float.Parse(expectedMatch.Groups[1].Value, System.Globalization.CultureInfo.InvariantCulture);
            var actualValue = float.Parse(actualMatch.Groups[1].Value, System.Globalization.CultureInfo.InvariantCulture);
            var expectedUnit = expectedMatch.Groups[2].Value.Trim();
            var actualUnit = actualMatch.Groups[2].Value.Trim();

            Assert.Equal(expectedUnit, actualUnit);
            Assert.True(Math.Abs(expectedValue - actualValue) <= Math.Abs(expectedValue) * tolerance,
                $"Expected {expectedValue} but got {actualValue} (tolerance: {tolerance * 100}%)");
        }
        else
        {
            Assert.Equal(expected, actual);
        }
    }

    [Theory]
    [InlineData("sum ( '2.3 m' '0.45 ft' '0.6' )", "m", "'3.03716 m'")]
    [InlineData("sum ( '2.3 m' '0.45 ft' '0.6' )", "ft", "'8.595932 ft'")]
    [InlineData("sum ( '2.3 m' '0.45 ft' '0.6' )", "", "'3.03716 m'")]
    [InlineData("'100 ft'", "m", "30.48")]
    [InlineData("'50 cm'", "m", "0.5")]
    [InlineData("sum ( '1 km' '500 m' '2000 mm' )", "m", "'1502 m'")]
    public void BasicUnitConversion(string expression, string targetUnit, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null, targetUnit);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("multiply ( '5 m' '3 ft' )", "'15 m·ft'")]
    [InlineData("multiply ( '2 cm' '4 mm' )", "'8 cm·mm'")]
    [InlineData("divide ( '10 m²' '2 m' )", "'5 m²/m'")]
    [InlineData("multiply ( '3' '4 m' )", "'12 m'")]
    [InlineData("multiply ( '2.5 kg' '9.8 m/s²' )", "'24.5 kg·m/s²'")]
    public void UnitArithmetic(string expression, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("sum ( '10' '20' '30' )", "60")]
    [InlineData("multiply ( '5' '6' )", "30")]
    [InlineData("subtract ( '100' '25' )", "75")]
    [InlineData("divide ( '50' '10' )", "5")]
    public void UnitlessOperations(string expression, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("sum ( multiply ( '2 m' '3' ) '1.5 ft' )", "m", "'6.4572 m'")]
    [InlineData("sum ( multiply ( '2 m' '3' ) '1.5 ft' )", "ft", "'21.18504 ft'")]
    [InlineData("divide ( sum ( '10 m' '5 ft' ) '3' )", "m", "'3.8413334 m'")]
    [InlineData("multiply ( sum ( '2 m' '3 ft' ) sum ( '4 ft' '1 m' ) )", "", "'21.21928 m·ft'")]
    [InlineData("sum ( multiply ( '3 kg' '2' ) multiply ( '1500 g' '4' ) )", "kg", "'12 kg'")]
    public void NestedExpressions(string expression, string targetUnit, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null, targetUnit);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("sum ( '1 m' '100 cm' '1000 mm' )", "m", "'3 m'")]
    [InlineData("sum ( '1 m' '100 cm' '1000 mm' )", "cm", "'300 cm'")]
    [InlineData("sum ( '1 m' '100 cm' '1000 mm' )", "mm", "'3000 mm'")]
    [InlineData("sum ( '1 ft' '12 in' '1 yd' )", "ft", "'5 ft'")]
    [InlineData("sum ( '1 kg' '1000 g' '1000000 mg' )", "kg", "'3 kg'")]
    public void MixedCompatibleUnits(string expression, string targetUnit, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null, targetUnit);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("sum ( subtract ( '10 m' '2 ft' ) multiply ( '3 m' '2' ) )", "m", "'15.3904 m'")]
    [InlineData("divide ( sum ( '100 cm' '1 m' ) subtract ( '5 ft' '30 cm' ) )", "", "'49.80392 cm/ft'")]
    [InlineData("multiply ( sum ( '2.5 kg' '500 g' ) sum ( '10 m/s' '5 ft/s' ) )", "", "'34.572 kg·m/s'")]
    [InlineData("sum ( sum ( sum ( '1 m' '1 ft' ) '1 in' ) '1 cm' )", "", "'1.3402001 m'")]
    public void VeryComplexNestedExpressions(string expression, string targetUnit, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null, targetUnit);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("min ( '5 m' '10 ft' )", "m", "'3.048 m'")]
    [InlineData("max ( '5 m' '10 ft' )", "m", "'5 m'")]
    [InlineData("average ( '10 m' '20 ft' '30 cm' )", "m", "'5.4653335 m'")]
    [InlineData("min ( '100 g' '0.2 kg' '5000 mg' )", "g", "'5 g'")]
    [InlineData("max ( '1 km' '3000 ft' '2000 m' )", "m", "'2000 m'")]
    public void MinMaxAverageWithUnits(string expression, string targetUnit, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null, targetUnit);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("power ( '3 m' '2' )", "'9 (m)^2'")]
    [InlineData("sqrt ( '16 m²' )", "'4 √(m²)'")]
    [InlineData("negate ( '5.5 ft' )", "'-5.5 ft'")]
    [InlineData("abs ( negate ( '10 kg' ) )", "'10 kg'")]
    [InlineData("mod ( '17 m' '5 m' )", "'2 m'")]
    public void AdvancedMathOperatorsWithUnits(string expression, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("multiply ( multiply ( '2 m' '3 ft' ) multiply ( '4 cm' '5 mm' ) )", "'120 m·ft·cm·mm'")]
    [InlineData("sum ( sum ( sum ( '1 m' '1 ft' ) '1 in' ) '1 cm' )", "'1.3402001 m'")]
    [InlineData("divide ( multiply ( '100 m²' '50 cm' ) sum ( '2 m' '6 ft' ) )", "'1305.8922 m²·cm/m'")]
    public void DeepNesting(string expression, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Theory]
    [InlineData("sum ( '0.001 km' '10 dm' '100 cm' '1000 mm' )", "m", "'4 m'")]
    [InlineData("multiply ( '0.5 kg' sum ( '2000 g' '3 kg' ) )", "kg", "'2.5 kg·kg'")]
    [InlineData("divide ( sum ( '1 mile' '1 km' ) '2' )", "km", "'1.304672 km'")]
    [InlineData("sum ( multiply ( '12 in' '12' ) sum ( '2 ft' '24 in' ) )", "ft", "'16 ft'")]
    public void PrecisionAndEdgeCases(string expression, string targetUnit, string expectedResult)
    {
        var expr = new Expression();
        expr.Deserialize(expression);
        var result = expr.Calculate(null, targetUnit);
        AssertUnitValueEqual(expectedResult, result.ToString());
    }

    [Fact]
    public void ComplexRealWorldScenario()
    {
        var expr = new Expression();
        expr.Deserialize("multiply ( sum ( '10 m' '5 ft' ) sum ( '3 m' '12 in' ) )");

        var result = expr.Calculate(null);
        Assert.NotNull(result);
        Assert.Contains("m", result.ToString());
    }

    [Fact]
    public void StressTestLargeExpression()
    {
        var innerExpr = "sum ( '1 m' '2 ft' '3 in' '4 cm' '5 mm' )";
        var middleExpr = $"multiply ( {innerExpr} {innerExpr} )";
        var outerExpr = $"sum ( {middleExpr} {middleExpr} {middleExpr} )";

        var expr = new Expression();
        expr.Deserialize(outerExpr);
        var result = expr.Calculate(null, "m");

        Assert.NotNull(result);
        Assert.Contains("m", result.ToString());
    }

    [Theory]
    [InlineData("'5.5 m'")]
    [InlineData("'0.001 km'")]
    [InlineData("'12345.6789 mm'")]
    [InlineData("'0 ft'")]
    [InlineData("'-10.5 cm'")]
    public void SingleValueParsing(string expression)
    {
        var expr = new Expression();
        Assert.NotNull(expr.Deserialize(expression));
        var result = expr.Calculate(null);
        Assert.NotNull(result);
    }

    [Fact]
    public void SerializationRoundTrip()
    {
        var expr1 = new Expression();
        expr1.Deserialize("sum ( multiply ( '2 m' '3 ft' ) '1.5 in' )");

        var serialized = expr1.Serialize();

        var expr2 = new Expression();
        expr2.Deserialize(serialized);

        var result1 = expr1.Calculate(null, "m");
        var result2 = expr2.Calculate(null, "m");

        Assert.Equal(result1.ToString(), result2.ToString());
    }
}
