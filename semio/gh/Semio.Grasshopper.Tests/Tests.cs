#region 🔖Header
// [👤semio📚gh🛅semiograsshoppertests💻tests](semiorepo://p/u/semio/b/l/gh/fd/req/Semio.Grasshopper.Tests/f/Tests.cs)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#endregion 🔖Header

using System.Drawing;
using Semio.Grasshopper;

namespace Semio.Grasshopper.Tests;

#region 🔖IconResourceTests
// Tests MUST verify icon resolution supports renamed keys and placeholder fallback.
public class IconResourceTests
{
    [Fact]
    public void ResolveOrPlaceholder_ShouldResolveExistingIcon()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("semio_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }

    [Fact]
    public void ResolveOrPlaceholder_ShouldResolveLegacyAliasWithoutUnderscore()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("attributeid_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }

    [Fact]
    public void ResolveOrPlaceholder_ShouldReturnPlaceholderWhenResourceIsMissing()
    {
        var bitmap = IconResources.ResolveOrPlaceholder("missing_resource_24x24");
        Assert.NotNull(bitmap);
        Assert.Equal(24, bitmap.Width);
        Assert.Equal(24, bitmap.Height);
    }
}
#endregion 🔖IconResourceTests
