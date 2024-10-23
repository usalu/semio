using Xunit;
using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.IO;

namespace Semio.UnitTests
{
    //public class DesignTests
    //{
    //    [Fact]
    //    public void Flatten_DefaultDesign_ReturnsDefaultDesign()
    //    {
    //        Design design = new Design();
    //        var flatten = design.Flatten();
    //        Assert.Equal(design, flatten);
    //    }

    //    public static IEnumerable<object[]> GetFlatDeepCloneDesigns()
    //    {
    //        yield return new object[]
    //        {
    //            new Design()
    //            {
    //                Pieces = new List<Piece>()
    //                {
    //                    new Piece()
    //                    {
    //                        Id = "1"
    //                    }
    //                }
    //            }
    //        };
    //        yield return new object[]
    //        {
    //            new Design()
    //            {
    //                Pieces = new List<Piece>()
    //                {
    //                    new Piece()
    //                    {
    //                        Id = "1",
    //                        Root = new PieceRoot()
    //                        {
    //                            Plane = new Plane()
    //                            {
    //                                Origin = new Point()
    //                                {
    //                                    X = 0,
    //                                    Y = 0,
    //                                    Z = 0
    //                                },
    //                                XAxis = new Vector()
    //                                {
    //                                    X = 1,
    //                                    Y = 0,
    //                                    Z = 0
    //                                },
    //                                YAxis = new Vector()
    //                                {
    //                                    X = 0,
    //                                    Y = 1,
    //                                    Z = 0
    //                                }
    //                            }
    //                        }
    //                    }
    //                }
    //            }
    //        };
    //        yield return new object[]
    //        {
    //            new Design()
    //            {
    //                Pieces = new List<Piece>()
    //                {
    //                    new Piece()
    //                    {
    //                        Id = "1",
    //                        Root = new PieceRoot()
    //                        {
    //                            Plane = new Plane()
    //                            {
    //                                Origin = new Point()
    //                                {
    //                                    X = 0,
    //                                    Y = 0,
    //                                    Z = 0
    //                                },
    //                                XAxis = new Vector()
    //                                {
    //                                    X = 1,
    //                                    Y = 0,
    //                                    Z = 0
    //                                },
    //                                YAxis = new Vector()
    //                                {
    //                                    X = 0,
    //                                    Y = 1,
    //                                    Z = 0
    //                                }
    //                            }
    //                        }
    //                    },
    //                    new Piece()
    //                    {
    //                        Id = "a",
    //                        Root = new PieceRoot()
    //                        {
    //                            Plane = new Plane()
    //                            {
    //                                Origin = new Point()
    //                                {
    //                                    X = 27,
    //                                    Y = 6,
    //                                    Z = -5
    //                                },
    //                                XAxis = new Vector()
    //                                {
    //                                    X = 0,
    //                                    Y = 1,
    //                                    Z = 0
    //                                },
    //                                YAxis = new Vector()
    //                                {
    //                                    X = 0,
    //                                    Y = 0,
    //                                    Z = -1
    //                                }
    //                            }
    //                        }
    //                    }
    //                }
    //            }
    //        };
    //    }

    //    [Theory]
    //    [MemberData(nameof(GetFlatDeepCloneDesigns))]
    //    public void Flatten_FlatDesign_ReturnsDeepClone(Design design)
    //    {
    //        var flatten = design.Flatten();
    //        Assert.Same(design.DeepClone(), flatten);
    //    }

    //    [Theory]
    //    [InlineData("../../tests/design_complex.json", "../../design_complex_flat.json")]
    //    public void Flatten_ComplexDesign(string designPath, string flattenedDesignPath)
    //    {
    //        var designJson = File.ReadAllText(designPath);
    //        var design = JsonConvert.DeserializeObject<Design>(designJson);

    //        var expectedFlattenedJson = File.ReadAllText(flattenedDesignPath);
    //        var expectedFlattenedDesign = JsonConvert.DeserializeObject<Design>(expectedFlattenedJson);

    //        var flattenedDesign = design.Flatten();

    //        Assert.Equal(expectedFlattenedDesign, flattenedDesign);
    //    }
    //}

    //public class KitTests
    //{
    //    [Theory]
    //    [InlineData("../../../../tests/kit_complex.json")]
    //    public void ComplexKit(string kitPath)
    //    {
    //        var kitJson = File.ReadAllText(kitPath);
    //        var kit = JsonConvert.DeserializeObject<Kit>(kitJson);
    //        var kitDeepClone = kit.DeepClone();
    //        Assert.Equal(kit, kitDeepClone);
    //        Assert.Equal(JsonConvert.SerializeObject(kit), JsonConvert.SerializeObject(kitDeepClone));
    //    }
    //    [Theory]
    //    [InlineData("../../../../tests/kit_invalid.json")]
    //    public void InvalidKit(string kitPath)
    //    {
    //        var kitJson = File.ReadAllText(kitPath);
    //        var kit = JsonConvert.DeserializeObject<Kit>(kitJson);

    //    }
    //}
}