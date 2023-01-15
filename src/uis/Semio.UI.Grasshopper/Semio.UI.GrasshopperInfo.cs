using Grasshopper;
using Grasshopper.Kernel;
using System;
using System.Drawing;

namespace Semio.UI.Grasshopper
{
    public class Semio_UI_GrasshopperInfo : GH_AssemblyInfo
    {
        public override string Name => "Semio";

        //Return a 24x24 pixel bitmap to represent this GHA library.
        public override Bitmap Icon => null;

        //Return a short string describing the purpose of this GHA library.
        public override string Description => "";

        public override Guid Id => new Guid("ecc07d8a-d211-4ca5-b066-109a60c5dc5d");

        //Return a string identifying you or your company.
        public override string AuthorName => "";

        //Return a string representing your preferred contact details.
        public override string AuthorContact => "";
    }
}