using System;
using System.Drawing;
using Grasshopper.Kernel;

namespace Semio.Grasshopper;

public class Semio_GrasshopperInfo : GH_AssemblyInfo
{
    public override string Name => "Semio.Grasshopper";

    //Return a 24x24 pixel bitmap to represent this GHA library.
    public override Bitmap Icon => null;

    //Return a short string describing the purpose of this GHA library.
    public override string Description => "Grasshopper user interface for semio.";

    public override Guid Id => new("FE587CBF-5F7D-4091-AA6D-D9D30CF80B64");

    public override string Version => "2.0.0";

    //Return a string identifying you or your company.
    public override string AuthorName => "Ueli Saluz";

    //Return a string representing your preferred contact details.
    public override string AuthorContact => "semio-community@posteo.org";
}