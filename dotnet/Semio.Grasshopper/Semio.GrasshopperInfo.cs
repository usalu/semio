using System;
using System.Drawing;
using Grasshopper;
using Grasshopper.Kernel;

namespace Semio.Grasshopper;

public class Semio_GrasshopperInfo : GH_AssemblyInfo
{
    public override string Name => "semio";
    public override Bitmap Icon => Resources.semio_24x24;
    public override Bitmap AssemblyIcon => Resources.semio_24x24;
    public override string Description => "semio within 🦗.";
    public override Guid Id => new("FE587CBF-5F7D-4091-AA6D-D9D30CF80B64");
    public override string Version => "3.0.1";
    public override string AuthorName => "Ueli Saluz";
    public override string AuthorContact => "semio-community@posteo.org";
}

public class SemioCategoryIcon : GH_AssemblyPriority
{
    public override GH_LoadingInstruction PriorityLoad()
    {
        Instances.ComponentServer.AddCategoryIcon("semio", Resources.semio_24x24);
        Instances.ComponentServer.AddCategorySymbolName("semio", 'S');
        return GH_LoadingInstruction.Proceed;
    }
}