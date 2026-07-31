#region 📱️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Unit tests for the Compose.Rhino bridge registry and layer service.

#endregion 📱️Header

#region ⌛️Imports
using Xunit;
using Newtonsoft.Json.Linq;
using Compose.Rhino;
#endregion ⌛️Imports

#region 🎩️BridgeRegistryTests
// Tests for BridgeRegistry routing, error handling, and method dispatch.

public class BridgeRegistryTests
{
    [Fact]
    public async Task HandleAsync_ReturnsBindingNotFound_ForUnregisteredBinding()
    {
        var registry = new BridgeRegistry();
        var request = new BridgeRequest { Id = "1", Binding = "unknown", Method = "test" };

        var response = await registry.HandleAsync(request);

        Assert.False(response.Ok);
        Assert.NotNull(response.Error);
        Assert.Equal("BINDING_NOT_FOUND", response.Error!.Code);
    }

    [Fact]
    public async Task HandleAsync_ReturnsMethodNotFound_ForUnregisteredMethod()
    {
        var registry = new BridgeRegistry();
        registry.Register(new AppBinding());
        var request = new BridgeRequest { Id = "2", Binding = "app", Method = "nonexistent" };

        var response = await registry.HandleAsync(request);

        Assert.False(response.Ok);
        Assert.NotNull(response.Error);
        Assert.Equal("METHOD_NOT_FOUND", response.Error!.Code);
    }

    [Fact]
    public async Task HandleAsync_AppPing_ReturnsPong()
    {
        var registry = new BridgeRegistry();
        registry.Register(new AppBinding());
        var request = new BridgeRequest { Id = "3", Binding = "app", Method = "ping" };

        var response = await registry.HandleAsync(request);

        Assert.True(response.Ok);
        Assert.Equal("pong", response.Result);
    }

    [Fact]
    public async Task HandleAsync_AppGetVersion_ReturnsVersion()
    {
        var registry = new BridgeRegistry();
        registry.Register(new AppBinding());
        var request = new BridgeRequest { Id = "4", Binding = "app", Method = "getVersion" };

        var response = await registry.HandleAsync(request);

        Assert.True(response.Ok);
        Assert.Equal(Constants.Version, response.Result);
    }

    [Fact]
    public void GetBindingNames_ReturnsRegisteredBindings()
    {
        var registry = new BridgeRegistry();
        registry.Register(new AppBinding());

        var names = registry.GetBindingNames().ToList();

        Assert.Contains("app", names);
    }

    [Fact]
    public void GetMethodNames_ReturnsRegisteredMethods()
    {
        var registry = new BridgeRegistry();
        registry.Register(new AppBinding());

        var methods = registry.GetMethodNames("app").ToList();

        Assert.Contains("ping", methods);
        Assert.Contains("getVersion", methods);
        Assert.Contains("getBridgeInfo", methods);
    }
}

#endregion 🎩️BridgeRegistryTests

#region 🛕️LayerServiceTests
// Tests for LayerService path construction.

public class LayerServiceTests
{
    [Fact]
    public void BuildRepresentationLayerPath_WithTags_ReturnsCorrectPath()
    {
        var path = LayerService.BuildRepresentationLayerPath("Metabolism", "CapsuleA", new[] { "floor", "unit" });
        Assert.Equal("compose::Metabolism::Types::CapsuleA::Representations::floor-unit", path);
    }

    [Fact]
    public void BuildRepresentationLayerPath_WithEmptyTags_ReturnsDefault()
    {
        var path = LayerService.BuildRepresentationLayerPath("Metabolism", "CapsuleA", Array.Empty<string>());
        Assert.Equal("compose::Metabolism::Types::CapsuleA::Representations::default", path);
    }

    [Fact]
    public void BuildRepresentationLayerPath_WithSingleTag_ReturnsCorrectPath()
    {
        var path = LayerService.BuildRepresentationLayerPath("MyKit", "Wall", new[] { "exterior" });
        Assert.Equal("compose::MyKit::Types::Wall::Representations::exterior", path);
    }
}

#endregion 🛕️LayerServiceTests

#region 🌀️BridgeProtocolTests
// Tests for BridgeRequest/BridgeResponse serialization.

public class BridgeProtocolTests
{
    [Fact]
    public void BridgeRequest_DefaultValues_AreCorrect()
    {
        var request = new BridgeRequest();
        Assert.Equal("", request.Id);
        Assert.Equal("", request.Binding);
        Assert.Equal("", request.Method);
        Assert.Null(request.Params);
    }

    [Fact]
    public void BridgeResponse_SuccessResponse_HasOkTrue()
    {
        var response = new BridgeResponse { Id = "1", Ok = true, Result = "test" };
        Assert.True(response.Ok);
        Assert.Equal("test", response.Result);
        Assert.Null(response.Error);
    }

    [Fact]
    public void BridgeResponse_ErrorResponse_HasErrorDetails()
    {
        var response = new BridgeResponse
        {
            Id = "1",
            Ok = false,
            Error = new BridgeError { Code = "TEST_ERROR", Message = "Something failed" }
        };
        Assert.False(response.Ok);
        Assert.NotNull(response.Error);
        Assert.Equal("TEST_ERROR", response.Error!.Code);
    }

    [Fact]
    public void ImportRepresentationRequest_DefaultValues_AreCorrect()
    {
        var request = new ImportRepresentationRequest();
        Assert.Equal("", request.KitName);
        Assert.Equal("", request.TypeName);
        Assert.Equal("", request.RepresentationId);
        Assert.Equal("", request.FileUrl);
        Assert.NotNull(request.Tags);
        Assert.Empty(request.Tags);
    }
}

#endregion 🌀️BridgeProtocolTests
