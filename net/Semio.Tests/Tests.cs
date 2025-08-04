using Newtonsoft.Json;

namespace Semio.Tests;

public class KitTests
{
    [Theory]
    [InlineData("../../../../../assets/semio/kit_metabolism.json")]
    public void ComplexKit(string kitPath)
    {
        var kitJson = System.IO.File.ReadAllText(kitPath);
        var kit = JsonConvert.DeserializeObject<Kit>(kitJson);
        var kitDeepClone = kit.DeepClone();
        Assert.Equal(kit, kitDeepClone);
        Assert.Equal(JsonConvert.SerializeObject(kit), JsonConvert.SerializeObject(kitDeepClone));
    }
}

public class ExpressionUnitTests
{
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
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
        Assert.Equal(expectedResult, result.ToString());
    }

    [Fact]
    public void ComplexRealWorldScenario()
    {
        // Complex nested expression with multiple operations and unit conversions
        var expr = new Expression();
        expr.Deserialize("multiply ( sum ( '10 m' '5 ft' ) sum ( '3 m' '12 in' ) )");

        var result = expr.Calculate(null);
        Assert.NotNull(result);
        Assert.Contains("m", result.ToString());
    }

    [Fact]
    public void StressTestLargeExpression()
    {
        // Build a very large nested expression programmatically
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
