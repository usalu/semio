using Grasshopper;
using Grasshopper.Kernel;
using System;
using System.Drawing;
using Semio.UI.Grasshopper.Properties;

namespace Semio.UI.Grasshopper
{
    public class Semio_UI_GrasshopperInfo : GH_AssemblyInfo
    {
        public override string Name => "Semio";
        public override Bitmap Icon => Resources.icon_semio;
        public override string Description => "Grasshopper user interface for Semio.";
        public override Guid Id => new Guid("ecc07d8a-d211-4ca5-b066-109a60c5dc5d");
        public override string AuthorName => "Ueli Saluz";
        public override string AuthorContact => "semio@posteo.org";
    }
}