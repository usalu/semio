using System;
using System.Collections.Generic;
using System.Linq;
using Grasshopper.Kernel;
using Grasshopper.Kernel.Types;
using Rhino.DocObjects;
using Rhino.Geometry;
using Semio.Model.V1;
using Semio.UI.Grasshopper.Goos;
using Semio.UI.Grasshopper.Params;
using Semio.UI.Grasshopper.Properties;
using Semio.UI.Grasshopper.Utility;

namespace Semio.UI.Grasshopper.Components.Utils
{
    public class ShowDesignComponent : GH_Component
    {
        public ShowDesignComponent()
          : base("Show Design", ":Design", "", "Semio", "Utils")
        {
        }
        protected override void RegisterInputParams(GH_Component.GH_InputParamManager pManager)
        {
            pManager.AddParameter(new DesignParam());
            pManager.AddTextParameter("Concepts", "C", "", GH_ParamAccess.list);
            pManager[1].Optional = true;
        }
        protected override void RegisterOutputParams(GH_Component.GH_OutputParamManager pManager)
        {
            pManager.AddGeometryParameter("Geometry","G","",GH_ParamAccess.list);
        }
        protected override void SolveInstance(IGH_DataAccess DA)
        {
            DesignGoo design = new();
            if (!DA.GetData(0, ref design)) return;

            var concepts = new List<string>();
            DA.GetDataList(1, concepts);

           
            // A dictionary with the prototype plan hash as key and the associated geometry as value.
            Dictionary<string, IEnumerable<GeometryBase>> prototypeGeometry = new();

            foreach (var prototype in design.Value.Prototypes)
            {
                foreach (var representation in prototype.Representations)
                {
                    if (representation.Concepts.Intersect(concepts).Any())
                    {
                        var goos = Converter.Convert(representation);
                        var geometricGoos = goos.Where(x => x is IGH_GeometricGoo).Cast<IGH_GeometricGoo>();
                        prototypeGeometry[prototype.PlanHash] = geometricGoos.Select(GH_Convert.ToGeometryBase);
                    }
                       
                }
            }

            var geometries = new List<GeometryBase>();

            foreach (var element in design.Value.Elements)
            {
                try
                {
                    var elementGeometries = prototypeGeometry[element.PrototypePlanHash]
                        .Select(g => g.Duplicate());
                    foreach (var elementGeometry in elementGeometries)
                        elementGeometry.Transform(
                            Transform.PlaneToPlane(Plane.WorldXY, Converter.Convert(element.Pose)));
                    geometries.AddRange(elementGeometries);
                }
                catch (Exception e)
                {
                    Console.WriteLine(e);
                    throw;
                }
            }

            DA.SetDataList(0,geometries);

        }
        protected override System.Drawing.Bitmap Icon => Resources.icon_show_design;
        public override Guid ComponentGuid => new ("464CD7FB-6994-4C81-B510-3EA5508CB650");
    }
}