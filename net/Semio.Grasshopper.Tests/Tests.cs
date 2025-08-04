using Newtonsoft.Json;
using System.Drawing;
using System.Numerics;
using Semio;
using Semio.Grasshopper;

namespace Semio.Grasshopper.Tests;

public class DesignTests
{
    //[Fact]
    //public void Flatten_DefaultDesign_ReturnsDefaultDesign()
    //{
    //    var design = new Design();
    //    var flatten = design.Flatten();
    //    Assert.Equal(design, flatten);
    //}

    //public static IEnumerable<object[]> GetFlatDeepCloneDesigns()
    //{
    //    yield return new object[]
    //    {
    //        new Design
    //        {
    //            Pieces = new List<Piece>
    //            {
    //                new()
    //                {
    //                    Id = "1"
    //                }
    //            }
    //        }
    //    };
    //    yield return new object[]
    //    {
    //        new Design
    //        {
    //            Pieces = new List<Piece>
    //            {
    //                new()
    //                {
    //                    Id = "1",

    //                    Plane = new Plane
    //                    {
    //                        Origin = new Point
    //                        {
    //                            X = 0,
    //                            Y = 0,
    //                            Z = 0
    //                        },
    //                        XAxis = new Vector
    //                        {
    //                            X = 1,
    //                            Y = 0,
    //                            Z = 0
    //                        },
    //                        YAxis = new Vector
    //                        {
    //                            X = 0,
    //                            Y = 1,
    //                            Z = 0
    //                        }
    //                    }
    //                }
    //            }
    //        }
    //    };
    //    yield return new object[]
    //    {
    //        new Design
    //        {
    //            Pieces = new List<Piece>
    //            {
    //                new()
    //                {
    //                    Id = "1",

    //                    Plane = new Plane
    //                    {
    //                        Origin = new Point
    //                        {
    //                            X = 0,
    //                            Y = 0,
    //                            Z = 0
    //                        },
    //                        XAxis = new Vector
    //                        {
    //                            X = 1,
    //                            Y = 0,
    //                            Z = 0
    //                        },
    //                        YAxis = new Vector
    //                        {
    //                            X = 0,
    //                            Y = 1,
    //                            Z = 0
    //                        }
    //                    }
    //                },
    //                new()
    //                {
    //                    Id = "a",

    //                    Plane = new Plane
    //                    {
    //                        Origin = new Point
    //                        {
    //                            X = 27,
    //                            Y = 6,
    //                            Z = -5
    //                        },
    //                        XAxis = new Vector
    //                        {
    //                            X = 0,
    //                            Y = 1,
    //                            Z = 0
    //                        },
    //                        YAxis = new Vector
    //                        {
    //                            X = 0,
    //                            Y = 0,
    //                            Z = -1
    //                        }
    //                    }
    //                }
    //            }
    //        }
    //    };
    //}

    //[Theory]
    //[MemberData(nameof(GetFlatDeepCloneDesigns))]
    //public void Flatten_FlatDesign_ReturnsDeepClone(Design design)
    //{
    //    var flatten = design.Flatten();
    //    Assert.Equal(design.DeepClone(), flatten);
    //}

    [Theory]
    [InlineData("../../../../../tests/kit_complex.json", "../../../../../tests/design_complex.json", "../../../../../tests/design_complex_flat.json")]
    public void Flatten_ComplexDesign(string kitPath, string designPath, string flattenedDesignPath)
    {
        var kitJson = System.IO.File.ReadAllText(kitPath);
        var kit = JsonConvert.DeserializeObject<Kit>(kitJson);

        var designJson = System.IO.File.ReadAllText(designPath);
        var design = JsonConvert.DeserializeObject<Design>(designJson);

        var expectedFlattenedDesignJson = System.IO.File.ReadAllText(flattenedDesignPath);
        var expectedFlattenedDesign = JsonConvert.DeserializeObject<Design>(expectedFlattenedDesignJson);

        var flattenedDesign = design.Flatten(kit.Types.ToArray(), Utility.ComputeChildPlane);

        Assert.Equal(expectedFlattenedDesign, flattenedDesign);
    }
}