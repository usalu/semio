using Newtonsoft.Json;

namespace Semio.Tests;

public class KitTests
{

    [Theory]
    [InlineData("../../../../../tests/kit_complex.json")]
    public void ComplexKit(string kitPath)
    {
        var kitJson = File.ReadAllText(kitPath);
        var kit = JsonConvert.DeserializeObject<Kit>(kitJson);
        var kitDeepClone = kit.DeepClone();
        Assert.Equal(kit, kitDeepClone);
        Assert.Equal(JsonConvert.SerializeObject(kit), JsonConvert.SerializeObject(kitDeepClone));
    }
}